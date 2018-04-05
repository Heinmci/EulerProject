use common::prime;
use rayon::prelude::*;

pub fn solve(number: u64) -> u64 {
    // Seems faster than reversing range, collecting and doing find_first (can't use rev on rayon Iter<u64>)
    (2..((number as f64).sqrt() as u64 + 1)).into_par_iter().find_last(|&x| number % x == 0 && prime::is_prime(x)).unwrap()
    //PrimeSequence::new().into_iter().take_while(|x| x <= &root).filter(|x| number % x == 0).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(600851475143);
        let end_time = PreciseTime::now();
        println!("Problem 3 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 6857);
    }
}