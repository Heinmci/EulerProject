pub fn is_prime(number: u64) -> bool {
    match number {
        0 | 1 => false,
        2 => true,
        x => {
            for i in  2..((x as f64).sqrt() as u64) + 1 {
                if x % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}