pub fn solve(nb: u32) -> u64 {
    get_nth_digit_fibo(nb)
}

pub fn get_nth_digit_fibo(nb: u32) -> u64 {
    if nb == 1 || nb == 2 {
        return 1;
    }

    let mut nb1 = vec![1];
    let mut nb2 = vec![2];
    let mut index: u64 = 3;
    loop {
        let next_number = sum_big_nunmbers(&nb1, &nb2);

        if next_number.len() == nb as usize {
            break;
        }

        nb1 = nb2;
        nb2 = next_number;
       
        index += 1;
    }
    index
}

fn sum_big_nunmbers(nb1: &Vec<u32>, nb2: &Vec<u32>) -> Vec<u32> {
    let mut sum = vec![];
    for index in 0..nb1.len() {
        sum.push(nb1[index] + nb2[index]);
    }

    if nb2.len() > nb1.len() {
        for index in nb1.len()..nb2.len() {
            sum.push(nb2[index]);
        }
    }

    clean_up_big_number(&mut sum);
    sum
}

fn clean_up_big_number(result: &mut Vec<u32>) {
    for index in 0.. {
        if *result.last().unwrap() < 10 {
            break;
        }

        let current_value = result[index];
        if current_value >= 10 {
            let modulo = current_value % 10;
            let division = current_value / 10;
            result[index] = modulo;

            if index == result.len() -1 {
                result.push(division);
            } else {
                result[index + 1] += division;
            }
        }
    }
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
        println!("Problem 25 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 4782);
    }
}
