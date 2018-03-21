pub fn solve(mut number: u32) -> u32 {
    let original_number = number;
    'outer: loop {
        for i in 1..21 {
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
    
    #[test]
    fn test_correct_result() {
        assert_eq!(solve(20), 232792560);
    }
}