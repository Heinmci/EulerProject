use common::fibonacci;

pub fn solve(nb: u32) -> usize {
    fibonacci::Fibonacci::new().into_iter().position(|x| x.len() == nb as usize).unwrap() + 1 // We add 1 because we start array at position 0
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
