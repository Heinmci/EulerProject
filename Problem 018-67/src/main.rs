use std::io::{BufReader,BufRead};
use std::fs::File;


fn main() {
    let mut pyramid = get_pyarmid_from_file("numbers_problem_67.txt");
    let solution = get_biggest_path(&mut pyramid);
    
    println!("{:?}", solution);
}

fn get_pyarmid_from_file(file_name: &str) -> Vec<Vec<u32>> {
    let mut pyramid = vec![];

    let file = File::open(file_name).expect("Couldn't open file");
    for line in BufReader::new(file).lines() {
        let mut row = vec![];
        for nb in line.unwrap().split(' ') {
            row.push(nb.parse::<u32>().unwrap());
        }
        pyramid.push(row);
    }
    pyramid
}

fn get_biggest_path(pyramid: &mut Vec<Vec<u32>>) -> u32 {
    for current_row in (1..(pyramid.len())).rev() {
        let nb_values_for_row = pyramid[current_row].len();
        
        for column in 0..(nb_values_for_row - 1) {
            let left_value = pyramid[current_row][column];
            let right_value = pyramid[current_row][column + 1];
            let top_value = pyramid[current_row - 1][column];
            
            if left_value > right_value {
                pyramid[current_row - 1][column] = top_value + left_value;
            } else {
                pyramid[current_row - 1][column] = top_value + right_value;
            }
        }
    }

    pyramid[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result_problem_18() {
        let mut pyramid = get_pyarmid_from_file("numbers_problem_18.txt");
        assert_eq!(get_biggest_path(&mut pyramid), 1074);
    }

    #[test]
    fn test_correct_result_problem_67() {
        let mut pyramid = get_pyarmid_from_file("numbers_problem_67.txt");
        assert_eq!(get_biggest_path(&mut pyramid), 7273);
    }
}