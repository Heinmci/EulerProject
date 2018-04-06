pub fn solve(mut number: u32) -> u32 {
    let original_number = number;
    let test: Vec<u32> = (1..21).rev().collect();
    'outer: loop {
        for i in &test {
            if number % i != 0 {
                number += original_number;
                continue 'outer;
            }
        }
        break;
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;

    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(20);
        let end_time = PreciseTime::now();
        println!("Problem 5 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 232792560);
    }
}