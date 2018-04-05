use std::mem;
use super::big_number;
use super::big_number::{BigNumber};

#[derive(Debug)]
pub struct Fibonacci {
    current: BigNumber,
    next: BigNumber,
}

impl Iterator for Fibonacci {
    type Item = BigNumber;
    
    fn next(&mut self) -> Option<BigNumber> {
        let mut return_current = BigNumber::new(0);
        let new_next = big_number::sum_big_numbers(&self.current, &self.next);
        // How to do this cleaner?
        mem::swap(&mut return_current, &mut self.current);
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

    #[test]
    fn test_bigger_fibo_sequence() {
        let fibo_sequence = Fibonacci::new().into_iter().take(7).collect::<Vec<BigNumber>>();
        assert_eq!(fibo_sequence[3].value(), &vec![3]);
        assert_eq!(fibo_sequence[4].value(), &vec![5]);
        assert_eq!(fibo_sequence[5].value(), &vec![8]);
        assert_eq!(fibo_sequence[6].value(), &vec![3, 1]);
    }
    //Fibonacci::new().into_iter().take(limit as usize).fold(BigNumber::new(0), |acc, x| big_number::sum_big_numbers(&acc, &x)).to_number()
    #[ignore]
    #[test]
    fn test_bigger_fibo_sequence2() {
        let fibo_sequence = Fibonacci::new().into_iter().take(100_000).fold(BigNumber::new(0), |acc, x| big_number::sum_big_numbers(&acc, &x)).to_number();
        assert_eq!(fibo_sequence, 17160773352933361367);
    }
}