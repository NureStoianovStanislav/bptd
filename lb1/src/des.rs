use crate::feistel;
use crate::keys::gen_keys;
use crate::permutation::*;

const BLOCK_SIZE: usize = core::mem::size_of::<u64>();

#[derive(Debug)]
pub enum DecryptError {
    InvalidLength,
    InvalidPadding,
}

pub fn encrypt(plaintext: &str, key: u64) -> Vec<u8> {
    let full_blocks = plaintext.as_bytes().windows(BLOCK_SIZE).step_by(BLOCK_SIZE);
    let rest_count = plaintext.len() % BLOCK_SIZE;
    let rest = plaintext
        .as_bytes()
        .iter()
        .rev()
        .take(rest_count)
        .rev()
        .copied()
        .chain(core::iter::repeat((BLOCK_SIZE - rest_count) as u8))
        .take(BLOCK_SIZE)
        .collect::<Vec<_>>();
    full_blocks
        .chain(core::iter::once(rest.as_slice()))
        .map(|block| u64::from_be_bytes(block.try_into().unwrap()))
        .map(|block| encrypt_block(block, key))
        .flat_map(u64::to_be_bytes)
        .collect()
}

pub fn decrypt(ciphertext: &[u8], key: u64) -> Result<Vec<u8>, DecryptError> {
    if ciphertext.len() % BLOCK_SIZE != 0 {
        return Err(DecryptError::InvalidLength);
    }
    let mut decrypted_bytes = ciphertext
        .windows(BLOCK_SIZE)
        .step_by(BLOCK_SIZE)
        .map(|block| u64::from_be_bytes(block.try_into().unwrap()))
        .map(|block| decrypt_block(block, key))
        .flat_map(u64::to_be_bytes)
        .collect::<Vec<_>>();
    let padding_len = match decrypted_bytes.last() {
        Some(&padding) if padding > 0 && padding <= BLOCK_SIZE as u8 => Ok(padding as usize),
        _ => Err(DecryptError::InvalidPadding),
    }?;
    decrypted_bytes.truncate(decrypted_bytes.len() - padding_len);
    Ok(decrypted_bytes)
}

pub fn encrypt_block(block: u64, key: u64) -> u64 {
    process(block, gen_keys(key))
}

pub fn decrypt_block(block: u64, key: u64) -> u64 {
    process(block, gen_keys(key).collect::<Vec<_>>().into_iter().rev())
}

fn process(block: u64, keys: impl Iterator<Item = u64>) -> u64 {
    let block = permute::<{ u64::BITS }>(block, &INITIAL_PERMUTATION);
    let initial_halves = bisect(block);
    let (l, r) = keys
        .scan(initial_halves, |state, k| {
            let (l, r) = *state;
            *state = feistel::round(l, r, k);
            Some(*state)
        })
        .last()
        .unwrap();
    let block = concat(l, r);
    permute::<{ u64::BITS }>(block, &FINAL_PERMUTATION)
}

fn concat(l: u32, r: u32) -> u64 {
    let mut block = <[u8; 8]>::default();
    l.to_be_bytes()
        .into_iter()
        .chain(r.to_be_bytes())
        .zip(block.iter_mut())
        .for_each(|(byte, w)| *w = byte);
    u64::from_be_bytes(block)
}

fn bisect(block: u64) -> (u32, u32) {
    let mut l = <[u8; 4]>::default();
    let mut r = <[u8; 4]>::default();
    r.iter_mut()
        .chain(l.iter_mut())
        .zip(block.to_be_bytes())
        .for_each(|(w, byte)| *w = byte);
    (u32::from_be_bytes(l), u32::from_be_bytes(r))
}
