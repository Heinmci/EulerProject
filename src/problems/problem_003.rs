use common;

pub fn solve(number: u64) -> u64 {
    for i in (2..((number as f64).sqrt() as u64 + 1)).rev() {
        if common::is_prime(i) && number % i == 0 {
            return i;
        }
    }

    return number;
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