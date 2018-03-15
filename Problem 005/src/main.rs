fn main() {
    let result = get_smallest_multiple(20);
    println!("Solution: {}", result);
}

fn get_smallest_multiple(mut number: u32) -> u32 {
    'outer: loop {
        for i in 1..21 {
            if number % i != 0 {
                number += 1;
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
        assert_eq!(get_smallest_multiple(20), 232792560);
    }
}