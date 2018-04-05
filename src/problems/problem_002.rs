use common::fibonacci::Fibonacci;
use common::big_number::BigNumber;
use common::big_number;

pub fn solve(limit: u64) -> u64 {
    Fibonacci::new().into_iter()
                    .take_while(|x| x.to_number() <= limit)
                    .filter(|x| x.is_even())
                    .fold(BigNumber::new(0), |acc, x| big_number::sum_big_numbers(&acc, &x))
                    .to_number()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(4000000);
        let end_time = PreciseTime::now();
        println!("Problem 2 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 4613732);
    }
    
}