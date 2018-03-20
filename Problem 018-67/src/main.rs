use std::cell::RefCell;
use std::io::{BufReader,BufRead};
use std::fs::File;


fn main() {
    let values = get_numbers_from_file("numbers_problem_67.txt");
    let mut pyramid = setup_pyramid(&values);
    let solution = get_biggest_path(&mut pyramid);
    
    println!("{:?}", solution);
}

fn get_numbers_from_file(file_name: &str) -> Vec<u32> {
    let mut numbers = vec![];

    let file = File::open(file_name).expect("Couldn't open file");
    for line in BufReader::new(file).lines() {
        for nb in line.unwrap().split(' ') {
            numbers.push(nb.parse::<u32>().unwrap());
        }
    }
    numbers
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

fn setup_pyramid(values: &[u32]) -> Vec<Vec<u32>> {
    let nb_rows = determine_nb_rows(values.len()).unwrap();
    let mut tiles = vec![];
    let mut current_index = 0;
    
    for current_row in 0..(nb_rows) {
        let mut row = vec![];
        for _ in 0..(current_row + 1) {
            row.push(values[current_index]);
            current_index += 1;
        }
        tiles.push(row);
    }

    tiles
}

fn determine_nb_rows(array_size: usize) -> Option<usize> {
    let mut nb_values = 0;

    for i in 0..array_size {
        nb_values += i;

        if nb_values == array_size {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result_problem_18() {
        let values = get_numbers_from_file("numbers_problem_18.txt");
        let mut pyramid = setup_pyramid(&values);
        assert_eq!(get_biggest_path(&mut pyramid), 1074);
    }

    #[test]
    fn test_correct_result_problem_67() {
        let values = get_numbers_from_file("numbers_problem_67.txt");
        let mut pyramid = setup_pyramid(&values);
        assert_eq!(get_biggest_path(&mut pyramid), 7273);
    }
}