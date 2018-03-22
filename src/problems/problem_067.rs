use problems::problem_018;

pub fn solve(pyramid: &mut Vec<Vec<u32>>) -> u32 {
    problem_018::solve(pyramid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let mut pyramid = problem_018::get_pyarmid_from_file("ressources/numbers_problem_067.txt");
        let result = solve(&mut pyramid);
        let end_time = PreciseTime::now();
        println!("Problem 67 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 7273);
    }
}