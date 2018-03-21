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
    
    #[test]
    fn test_correct_result() {
        assert_eq!(solve(600851475143), 6857);
    }
}