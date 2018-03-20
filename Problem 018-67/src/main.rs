use std::cell::RefCell;
use std::io::{BufReader,BufRead};
use std::fs::File;


fn main() {
    let values = get_numbers_from_file("numbers_problem_67.txt");
    let pyramid = setup_pyramid(&values);
    let solution = get_biggest_path(&pyramid);
    
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

fn get_biggest_path(pyramid: &Vec<Vec<PyramidTile>>) -> u32 {
    let pyramid_top = &pyramid[0][0];
    *pyramid_top.sum_to_this_point.borrow_mut() = pyramid_top.value;

    for current_row in 1..pyramid.len() {
        let nb_values_for_row = pyramid[current_row].len();
        for column in 0..nb_values_for_row {

            if column > 0 {
                deal_with_path(pyramid, current_row, column, Direction::TopLeft);
                
            }
            if column < nb_values_for_row -1 {
                deal_with_path(pyramid, current_row, column, Direction::TopRight);
            }
        }
    }

    get_max_value_in_last_row(pyramid)
}

fn get_max_value_in_last_row(pyramid: &Vec<Vec<PyramidTile>>) -> u32 {
    let nb_rows = pyramid.len();
    let mut max = *pyramid[nb_rows -1][0].sum_to_this_point.borrow();
    for column in 1..pyramid[nb_rows -1].len() {
        let sum_to_this_point = *pyramid[nb_rows -1][column].sum_to_this_point.borrow();
        if sum_to_this_point > max {
            max = sum_to_this_point;
        }
    }

    max
}

fn deal_with_path(pyramid: &Vec<Vec<PyramidTile>>, current_row: usize, column: usize, direction: Direction) {
    let current_tile = &pyramid[current_row][column];
    let sum_to_this_point = *current_tile.sum_to_this_point.borrow();
    let previous_row_col_number = {
        if direction == Direction::TopLeft {
            column -1
        } else {
            column
        }
    };

    let potential_new_sum = current_tile.value + *pyramid[current_row - 1][previous_row_col_number].sum_to_this_point.borrow();

    if potential_new_sum > sum_to_this_point {
        *current_tile.sum_to_this_point.borrow_mut() = potential_new_sum;
    }
}


fn setup_pyramid(values: &[u32]) -> Vec<Vec<PyramidTile>> {
    let nb_rows = determine_nb_rows(values.len()).unwrap();
    let mut tiles = vec![];
    let mut current_index = 0;
    
    for current_row in 0..(nb_rows) {
        let mut row = vec![];
        for _ in 0..(current_row + 1) {
            let tile = PyramidTile::new(values[current_index]);
            row.push(tile);
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

#[derive(Debug)]
struct PyramidTile {
    value: u32,
    sum_to_this_point: RefCell<u32>
}

impl PyramidTile {
    pub fn new(value: u32) -> PyramidTile {
        PyramidTile {
            value,
            sum_to_this_point: RefCell::new(0)
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    TopLeft,
    TopRight
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result_problem_18() {
        let values = get_numbers_from_file("numbers_problem_18.txt");
        let pyramid = setup_pyramid(&values);
        assert_eq!(get_biggest_path(&pyramid), 1074);
    }

    #[test]
    fn test_correct_result_problem_67() {
        let values = get_numbers_from_file("numbers_problem_67.txt");
        let pyramid = setup_pyramid(&values);
        assert_eq!(get_biggest_path(&pyramid), 7273);
    }
}