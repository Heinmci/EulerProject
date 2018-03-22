pub fn solve(limit: u32) -> u32 {
    let mut sum = 2;
    let mut nb_1 = 1;
    let mut nb_2 = 2;
    loop {
        let next_number = nb_1 + nb_2;
        if next_number > limit {
            break;
        }
        nb_1 = nb_2;
        nb_2 = next_number;
        if next_number%2 == 0 {
            sum += next_number;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(4000000);
        let end_time = PreciseTime::now();
        println!("Problem 2 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 4613732);
    }
}