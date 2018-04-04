use std::mem;
use super::big_number;
use super::big_number::{BigNumber};


pub struct Fibonacci {
    current: BigNumber,
    next: BigNumber,
}

impl Iterator for Fibonacci {
    type Item = BigNumber;
    
    fn next(&mut self) -> Option<BigNumber> {
        let return_current = self.current.clone(); // We start by returning first element of fibo sequence
        let new_next = big_number::sum_big_numbers(&self.current, &self.next);

        mem::swap(&mut self.current, &mut self.next);
        self.next = new_next;

        Some(return_current)
    }
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { current: BigNumber::new(1), next: BigNumber::new(1) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibo_sequence() {
        let fibo_sequence = Fibonacci::new().into_iter().take(3).collect::<Vec<BigNumber>>();
        assert_eq!(fibo_sequence[0].value(), &vec![1]);
        assert_eq!(fibo_sequence[1].value(), &vec![1]);
        assert_eq!(fibo_sequence[2].value(), &vec![2]);
    }

    #[test]
    fn test_other_fibo_sequence() {
        let fibo_sequence = Fibonacci::new().into_iter().skip(3).take(3).collect::<Vec<BigNumber>>();
        assert_eq!(fibo_sequence[0].value(), &vec![3]);
        assert_eq!(fibo_sequence[1].value(), &vec![5]);
        assert_eq!(fibo_sequence[2].value(), &vec![8]);
    }
}