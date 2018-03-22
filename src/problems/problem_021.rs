use std::collections::HashMap;
use common;

pub fn solve(limit: u32) -> u32 {
    let mut value_cache = HashMap::new();
    let mut amicable_numbers: Vec<u32> = vec![];

    for i in 2..limit {
        if amicable_numbers.contains(&i) {
            continue;
        }

        let sum_of_divisors = {
            if value_cache.get(&i).is_none() {
                value_cache.insert(i, common::get_proper_divisor_sum(i));
            } 
            value_cache[&i]
        };
        
        if sum_of_divisors == i {
            continue;
        }

        let divisor_sum_of_sum = {
            if value_cache.get(&sum_of_divisors).is_none() {
                value_cache.insert(sum_of_divisors, common::get_proper_divisor_sum(sum_of_divisors));
            } 
            value_cache[&sum_of_divisors]
        };


        if divisor_sum_of_sum == i {
            amicable_numbers.push(i);
            amicable_numbers.push(sum_of_divisors);
        }
    }

    amicable_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(10000);
        let end_time = PreciseTime::now();
        println!("Problem 21 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 31626);
    }
}
