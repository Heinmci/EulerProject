use common;

pub fn solve(limit: u32) -> u32 {
    let mut sum = 0;
    let abundant_numbers = get_abundant_numbers(limit);
    'outer: for i in 1..limit {
        for number in abundant_numbers.iter() {
            if *number > i {
                break;
            }

            if abundant_numbers.contains(&(i - number)) {
                continue 'outer;
            }
        }
        sum += i;
    }

    sum
}

fn get_abundant_numbers(limit: u32) -> Vec<u32> {
    let mut numbers = vec![];

    for i in 12..limit {
        if common::get_proper_divisor_sum(i) > i {
            numbers.push(i);
        }
    }

    numbers
}



#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(28123);
        let end_time = PreciseTime::now();
        println!("Problem 23 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 4179871);
    }
}
