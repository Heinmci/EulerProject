use common::prime::PrimeSequence;

pub fn solve(nb: usize) -> u64 {
    PrimeSequence::new().into_iter().nth(nb - 1).unwrap() // nth Starts at 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(10001);
        let end_time = PreciseTime::now();
        println!("Problem 7 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 104743);
    }
}
