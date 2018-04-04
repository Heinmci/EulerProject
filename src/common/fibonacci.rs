use std::mem;
use super::big_number;


pub struct Fibonacci {
    current: Vec<u32>,
    next: Vec<u32>,
}

impl Iterator for Fibonacci {
    type Item = Vec<u32>;
    
    fn next(&mut self) -> Option<Vec<u32>> {
        let return_current = self.current.clone(); // We start by returning first element of fibo sequence
        let new_next = big_number::sum_big_numbers(&self.current, &self.next);

        mem::swap(&mut self.current, &mut self.next);
        self.next = new_next;

        Some(return_current)
    }
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { current: vec![1], next: vec![1] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibo_sequence() {
        let fibo_sequence = Fibonacci::new().into_iter().take(3).collect::<Vec<Vec<u32>>>();
        assert_eq!(fibo_sequence, [[1], [1], [2]]);
    }

    #[test]
    fn test_other_fibo_sequence() {
        let fibo_sequence = Fibonacci::new().into_iter().skip(3).take(3).collect::<Vec<Vec<u32>>>();
        assert_eq!(fibo_sequence, [[3], [5], [8]]);
    }
}