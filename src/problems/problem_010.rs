use common::prime;
use rayon::prelude::*;

pub fn solve(limit: u64) -> u64 {
    (3..limit).into_par_iter().filter(|&x| prime::is_prime(x)).sum::<u64>() + 2
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