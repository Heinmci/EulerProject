use std::io::{BufReader,BufRead};
use std::fs::File;

pub fn get_pyarmid_from_file(file_name: &str) -> Vec<Vec<u32>> {
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

pub fn solve(pyramid: &mut Vec<Vec<u32>>) -> u32 {
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
    fn test_correct_result() {
        let mut pyramid = get_pyarmid_from_file("ressources/numbers_problem_018.txt");
        assert_eq!(solve(&mut pyramid), 1074);
    }

}