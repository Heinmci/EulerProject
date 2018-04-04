use common::big_number::BigNumber;

pub fn solve(power: u32) -> u32 {
    let mut result = BigNumber::new(2);
    
    for _ in 0..(power - 1) { // -1 parce que on a déjà donné le 2^1 au dessus
        result.mul_with_nb(2);
    }

    result.value().iter().fold(0u32,|acc,&b| acc + b as u32)
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
        println!("Problem 16 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 1366);
    }
}
