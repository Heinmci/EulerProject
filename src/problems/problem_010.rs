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
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(2000000);
        let end_time = PreciseTime::now();
        println!("Problem 10 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 142913828922);
    }
}