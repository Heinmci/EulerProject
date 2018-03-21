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
    
    #[test]
    fn test_correct_result() {
        assert_eq!(solve(10001), 104743);
    }
}
