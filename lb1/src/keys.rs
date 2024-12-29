use crate::{entropy::count_ones, permutation::permute};

const BITMAP_56: u64 = 0x0000FFFFFFFFFFFFFF;

const SHIFTS: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

const EXPANSION_TABLE_LEN: usize = 56;

const EXPANSION_TABLE: [u8; EXPANSION_TABLE_LEN] = [
    57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60,
    52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29,
    21, 13, 5, 28, 20, 12, 4,
];

const KEY_BITS: [u8; 48] = [
    14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41, 52,
    31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32,
];

pub fn gen_keys(key: u64) -> impl Iterator<Item = u64> {
    let expanded_key = expand_key(key);
    assert_eq!(expanded_key & BITMAP_56, expanded_key);
    (1..=SHIFTS.len())
        .map(|i| SHIFTS.iter().take(i).sum::<u8>())
        .map(move |n| {
            (
                // clear bits excluding last n
                expanded_key << (u64::BITS as u8 - n)
                // shift them back to new position (56 bit)
                >> (u64::BITS as usize - EXPANSION_TABLE_LEN)
            )
            // combine with rest
            | (expanded_key >> n)
        })
        .map(|vector| permute::<{ EXPANSION_TABLE_LEN as u32 }>(vector, &KEY_BITS))
}

pub fn seed_key(key: &str) -> u64 {
    let hash = const_fnv1a_hash::fnv1a_hash_str_64(key);
    let mut hash_bytes = hash.to_be_bytes();
    hash_bytes
        .iter_mut()
        .filter(|byte| count_ones(**byte as u64) & 1 == 0)
        .for_each(|byte| *byte ^= 1);
    u64::from_be_bytes(hash_bytes)
}

pub fn check_key_regression(key: u64) -> Result<u64, ()> {
    key.to_be_bytes()
        .iter()
        .all(|&byte| count_ones(byte as u64) & 1 == 1)
        .then_some(key)
        .ok_or(())
}

fn expand_key(key: u64) -> u64 {
    permute::<{ u64::BITS }>(key, &EXPANSION_TABLE)
}
