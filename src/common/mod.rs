pub fn is_prime(number: u64) -> bool {
    match number {
        0 | 1 => false,
        2 | 3 => true,
        x => {
            if x % 2 == 0 || x % 3 == 0 {
                return false;
            } 
            for i in  (5..((x as f64).sqrt() as u64) + 1).step_by(6) {
                if x % i == 0 || x % (i + 2) == 0 {
                    return false;
                }
            }
            true
        }
    }
}
