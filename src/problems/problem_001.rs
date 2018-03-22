pub fn solve(limit: u32) -> u32 {
    (1..limit).filter(|x| x%3 == 0 || x%5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;

    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(1000);
        let end_time = PreciseTime::now();
        println!("Problem 1 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 233168);
    }
}
