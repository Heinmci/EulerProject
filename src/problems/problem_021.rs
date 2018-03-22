use std::collections::HashMap;

pub fn solve(limit: u32) -> u32 {
    let mut value_cache = HashMap::new();
    let mut amicable_numbers: Vec<u32> = vec![];

    for i in 2..limit {
        if amicable_numbers.contains(&i) {
            continue;
        }

        value_cache.entry(i).or_insert(get_proper_divisor_sum(i));
        let sum_of_divisors = value_cache[&i];

        if sum_of_divisors == i {
            continue;
        }

        value_cache.entry(sum_of_divisors).or_insert(get_proper_divisor_sum(sum_of_divisors));
        let divisor_sum_of_sum = value_cache[&sum_of_divisors];

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
    
    #[test]
    fn test_correct_result() {
        assert_eq!(solve(10000), 31626);
    }
}
