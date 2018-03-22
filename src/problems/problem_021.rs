use std::collections::HashMap;

pub fn solve(limit: u32) -> u32 {
    let mut value_cache = HashMap::new();
    let mut amicable_numbers: Vec<u32> = vec![];

    for i in 2..limit {
        if amicable_numbers.contains(&i) {
            continue;
        }

        let sum_of_divisors = {
            if value_cache.get(&i).is_none() {
                value_cache.insert(i, get_proper_divisor_sum(i));
            } 
            value_cache[&i]
        };
        
        if sum_of_divisors == i {
            continue;
        }

        let divisor_sum_of_sum = {
            if value_cache.get(&sum_of_divisors).is_none() {
                value_cache.insert(sum_of_divisors, get_proper_divisor_sum(sum_of_divisors));
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

fn get_proper_divisor_sum(number: u32) -> u32 {
    let mut sum_divisors = 1;
    let square_root = (number as f32).sqrt() as u32;
    
    if number % square_root == 0 {
        sum_divisors += square_root;
    }

    for i in 2..square_root {
        if number % i == 0 {
            sum_divisors += i;
            sum_divisors += number / i;
        }
    }

    sum_divisors
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
