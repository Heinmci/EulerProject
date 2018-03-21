use problems::problem_018;

pub fn solve(pyramid: &mut Vec<Vec<u32>>) -> u32 {
    problem_018::solve(pyramid)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        let mut pyramid = problem_018::get_pyarmid_from_file("ressources/numbers_problem_067.txt");
        assert_eq!(solve(&mut pyramid), 7273);
    }
}