pub fn nth(n: u32) -> u32 {
    (2..104_744)
        .filter(|prime| is_prime(*prime))
        .nth(n as usize)
        .unwrap_or_default()
}

fn is_prime(n: u32) -> bool {
    !(2..n - 1).any(|i| n % i == 0)
}
