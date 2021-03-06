pub struct PrimeSequence {
    nth_prime: u64,
    current_prime_candidate: u64,
    candidate_identifier: Identifier,
    primes_generated: Vec<u64>
}

impl PrimeSequence {
    pub fn new() -> PrimeSequence {
        PrimeSequence {
            nth_prime: 0,
            current_prime_candidate: 5,
            candidate_identifier: Identifier::First,
            primes_generated: vec![2, 3]
        }
    }

    pub fn determine_next_candidate(&mut self) {
        if self.candidate_identifier == Identifier::First {
            self.current_prime_candidate += 2;
            self.candidate_identifier = Identifier::Second;
        } else {
            self.current_prime_candidate += 4;
            self.candidate_identifier = Identifier::First;
        }
    }
}

impl Iterator for PrimeSequence {
    type Item = u64;
    
    fn next(&mut self) -> Option<u64> {
        self.nth_prime += 1;
        
        if self.nth_prime == 1 {
            return Some(2);
        } else if self.nth_prime == 2 {
            return Some(3);
        }
        
        loop {
            if iterator_is_prime(self.current_prime_candidate, &self.primes_generated) {
                let prime = self.current_prime_candidate;
                self.determine_next_candidate();
                self.primes_generated.push(prime);
                return Some(prime);
            } else {
                self.determine_next_candidate();
            }
        }
    }
}

pub fn iterator_is_prime(x: u64, primes: &Vec<u64>) -> bool {
    for i in primes.iter().take_while(|a| **a <= ((x as f64).sqrt() as u64)) {
        if x % i == 0 {
            return false;
        }
    }
    true
}

pub fn is_prime(number: u64) -> bool {
    match number {
        0 | 1 => false,
        2 | 3 => true,
        x => {
            if x % 2 == 0 || x % 3 == 0 {
                return false;
            } 
            for i in  (5..((x as f64).sqrt() as u64) + 1).step_by(6) {
                if x % i == 0 || x % (i + 2) == 0 {
                    return false;
                }
            }
            true
        }
    }
}

#[derive(PartialEq, Debug)]
enum Identifier {
    First,
    Second
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_prime_sequence() {
        let prime_sequence = PrimeSequence::new().into_iter().take(3).collect::<Vec<u64>>();
        assert_eq!(prime_sequence, vec![2, 3, 5]);
    }

    #[test]
    fn test_other_prime_sequence() {
        let prime_sequence = PrimeSequence::new().into_iter().skip(3).take(3).collect::<Vec<u64>>();
        assert_eq!(prime_sequence, vec![7, 11, 13]);
    }

    #[test]
    fn get_millionth_prime() {
        let start_time = PreciseTime::now();
        let prime = PrimeSequence::new().into_iter().nth(999_999).unwrap();
        let end_time = PreciseTime::now();
        println!("Millionth prime took: {}", start_time.to(end_time));
        assert_eq!(prime, 15_485_863);
    }
}