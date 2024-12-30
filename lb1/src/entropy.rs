use crate::des;

pub fn calculate_entropy(bytes: &[u8]) -> f64 {
    let total_entropy: f64 = bytes
        .chunks(des::BLOCK_SIZE)
        .map(|block| {
            let block = block
                .iter()
                .copied()
                .chain(core::iter::repeat(0))
                .take(des::BLOCK_SIZE)
                .collect::<Vec<_>>();
            let block = u64::from_be_bytes(block.try_into().unwrap());
            match (count_ones(block) as f64) / (u64::BITS as f64) {
                0. | 1. => 0.,
                p => -p * p.log2() - (1. - p) * (1. - p).log2(),
            }
        })
        .sum();
    total_entropy / (bytes.len() as f64 / des::BLOCK_SIZE as f64).ceil()
}

pub fn count_ones(block: u64) -> u8 {
    core::iter::successors(Some(block), |b| match b >> 1 {
        0 => None,
        b => Some(b),
    })
    .map(|b| b & 1)
    .filter(|&bit| bit == 1)
    .count() as u8
}
