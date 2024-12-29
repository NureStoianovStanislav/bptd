pub fn calculate_entropy(bytes: &[u8]) -> f64 {
    let total = bytes
        .windows(8)
        .step_by(8)
        .map(|bytes| {
            let block = u64::from_be_bytes(bytes.try_into().unwrap());
            let p = (count_ones(block) as f64) / (u64::BITS as f64);
            -p * p.log2() - (1. - p) * (1. - p).log2()
        })
        .sum::<f64>();
    total / bytes.len() as f64
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
