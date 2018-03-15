fn main() {
    let number = 600851475143;
    let biggest_prime_factor = get_biggest_prime_factor(number);
    println!("Biggest prime factor of {} is {}", number, biggest_prime_factor);
}

fn get_biggest_prime_factor(number: u64) -> u64 {
    for i in (2..((number as f64).sqrt() as u64 + 1)).rev() {
        if is_prime(i) && number % i == 0 {
            return i;
        }
    }

    return number;
}

fn is_prime(number: u64) -> bool {
    match number {
        0 | 1 => false,
        2 => true,
        x => {
            for i in  2..((x as f64).sqrt() as u64) + 1 {
                if x % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        assert_eq!(get_biggest_prime_factor(600851475143), 6857);
    }
}