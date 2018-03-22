use std::io::{BufReader,BufRead};
use std::fs::File;

pub fn solve(file_name: &str) -> u64 {
    let mut names = get_names_from_file_name(file_name);
    names.sort(); 
    get_name_score_sum(&names)
}

fn get_name_score_sum(names: &[String]) -> u64 {
    let mut sum_score = 0;
    for (index, name) in names.iter().enumerate() {
        let name_score: u32 = name.chars().fold(0, |acc, letter| acc + (letter as u32 - 64));
        sum_score += name_score as u64 * (index as u64 + 1)
    }

    sum_score
}

fn get_names_from_file_name(file_name: &str) -> Vec<String>{
    let mut names = vec![];

    let file = File::open(file_name).expect("Couldn't open file");
    for line in BufReader::new(file).lines() {
        for element in line.unwrap().split(",") {
            let name: String = element.chars().filter(|&x| x != '\\' && x != '"').collect();
            names.push(name);
        }
    }
    names
}


#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve("ressources/names_problem_022.txt");
        let end_time = PreciseTime::now();
        println!("Problem 22 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 871198282);
    }
}
