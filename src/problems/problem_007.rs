use common;

pub fn solve(nb: u32) -> u64 {
    if nb == 1 {
        return 2;
    }
    let mut counter = 3;
    let mut nb_primes = 1;
    loop {
        if common::is_prime(counter) {
            nb_primes += 1;
            if nb_primes == nb {
                return counter;
            }
        }
        counter += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(10001);
        let end_time = PreciseTime::now();
        println!("Problem 7 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 104743);
    }
}
