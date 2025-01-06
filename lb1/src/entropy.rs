pub fn calculate_entropy(block: u64) -> f64 {
    match (count_ones(block) as f64) / (u64::BITS as f64) {
        0. | 1. => 0.,
        p => -p * p.log2() - (1. - p) * (1. - p).log2(),
    }
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
