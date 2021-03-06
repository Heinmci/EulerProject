use std::io::{BufReader,BufRead};
use std::fs::File;

pub fn solve(numbers: [[u32; 50]; 100]) -> u64 {
    let mut sum = [0; 55];
    let mut result: u64 = 0;

    for i in 0..50 {
        for j in 0..100 {
            sum[i] += numbers[j][i];
        }
    }
    let array_length = clean_up_array(&mut sum);

    for i in (0..10).rev() {
        result += (sum[array_length - (10 - i)]) as u64 * 10u64.pow(i as u32);
    }
    result
}

pub fn get_numbers_from_file(file_name: &str) -> [[u32; 50]; 100] {
    let mut numbers: [[u32; 50]; 100] = [[0; 50]; 100];

    let file = File::open(file_name).expect("Couldn't open file");
    for (i, line) in BufReader::new(file).lines().enumerate() {
        for (j, digit) in line.unwrap().chars().rev().enumerate() {
            numbers[i][j] = digit.to_string().parse::<u32>().unwrap();
        }
    }
    numbers
}

fn clean_up_array(result: &mut [u32; 55]) -> usize {
    let mut last_index = 0;
    for k in 0..55 {
        let current_value = result[k];
        if current_value >= 10 {
            let modulo = current_value % 10;
            let division = current_value / 10;
            result[k] = modulo;
            result[k + 1] += division;
        }

        if result[k] != 0 {
            last_index = k;
        }
    }
    last_index + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let numbers = get_numbers_from_file("ressources/numbers_problem_013.txt");
        let result = solve(numbers);
        let end_time = PreciseTime::now();
        println!("Problem 13 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 5537376230);
    }
}
