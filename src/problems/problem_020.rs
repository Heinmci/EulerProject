use common::big_number::BigNumber;

pub fn solve(factorial: u64) -> u64 {
    let mut result = BigNumber::new(1);
    
    for nb in 1..(factorial + 1) {
        result.mul_with_nb(nb as u32);
    }
    result.value().iter().fold(0u64,|acc,&b| acc + b as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(100);
        let end_time = PreciseTime::now();
        println!("Problem 20 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 648);
    }
}
