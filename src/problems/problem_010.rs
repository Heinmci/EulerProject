use common;

pub fn solve(limit: u64) -> u64 {
    let mut sum_of_primes = 2;
    for counter in (3..limit).step_by(2) {
        if common::is_prime(counter) {
            sum_of_primes += counter;
        }
    }
    sum_of_primes
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        assert_eq!(solve(2000000), 142913828922);
    }
}