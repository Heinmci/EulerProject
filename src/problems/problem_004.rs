pub fn solve(limit1: u32) -> u32 {
    let mut max = 0;
    for i in (0..limit1 + 1).rev() {
        for j in (0..i + 1).rev() {
            let mult = i * j;
            if  mult > max && is_palydrome(mult) {
                max = mult;
            }
        }
    }
    max
}

fn is_palydrome(number: u32) -> bool {
    let number_str = number.to_string();
    number_str == number_str.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(999);
        let end_time = PreciseTime::now();
        println!("Problem 4 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 906609);
    }
}