use common;

pub fn solve(limit: u32) -> u32 {
    let mut sum = limit * (limit + 1) / 2;
    let mut was_subtracted: Vec<bool> = vec![false; limit as usize + 1];
    let abundant_numbers = get_abundant_numbers(limit);
    
    for i in abundant_numbers.iter() {
        for j in abundant_numbers.iter() {
            let tmp_sum = i + j;
            if tmp_sum <= limit && !was_subtracted[tmp_sum as usize] {
                was_subtracted[tmp_sum as usize] = true;
                sum -= tmp_sum;
            }
        }
    }
 
    sum
}

fn get_abundant_numbers(limit: u32) -> Vec<u32> {
    let mut numbers = vec![];

    for i in 12..(limit + 1) {
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
