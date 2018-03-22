use common;

pub fn solve(limit: u32) -> u32 {
    let mut sum = 0;
    let abundant_numbers = get_abundant_numbers(limit);
    println!("{:?}", abundant_numbers);
    /*'outer: for i in 1..limit {
        for number in abundant_numbers.iter() {
            if *number > i {
                break;
            }

            if abundant_numbers.contains(&(i - number)) {
                continue 'outer;
            }
        }
        sum += i;
    }*/

    for i in 1..limit {
        sum += i;
    }
    let mut abundant_pair_sum: Vec<u32> = vec![];
    for i in abundant_numbers.iter() {
        for j in abundant_numbers.iter() {
            let tmp_sum = i + j;
            if tmp_sum < limit && !abundant_pair_sum.contains(&tmp_sum) {
                abundant_pair_sum.push(tmp_sum);
            }
        }
    }

    sum - (abundant_pair_sum.iter().sum()) as u32
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
