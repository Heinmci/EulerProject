#[derive(Debug, Clone)]
pub struct BigNumber {
    value: Vec<u32>
}

impl BigNumber {
    pub fn new(nb: u64) -> BigNumber {
        BigNumber {
            value: nb.to_string().chars().rev().map(|d| d.to_digit(10).unwrap() as u32).collect()
        }
    }

    pub fn new_from_vec(value: Vec<u32>) -> BigNumber {
        BigNumber {
            value
        }
    }

    pub fn new_from_string(nb: &str) -> BigNumber {
        BigNumber {
            value: nb.chars().rev().map(|d| d.to_digit(10).unwrap() as u32).collect()
        }
    }

    pub fn value(&self) -> &Vec<u32> {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Vec<u32> {
        &mut self.value
    }

    pub fn is_even(&self) -> bool {
        self.value()[0] % 2 == 0
    }

    // TODO: Make this return Option<u64> if nb > <u64>::max_value()
    pub fn to_number(&self) -> u64 {
        let mut result = 0;
        for (index, value) in self.value.iter().enumerate() {
            result += *value as u64 * 10u64.pow(index as u32);
        }

        result
    }

    pub fn mul_with_nb(&mut self, nb: u32) {
        //self.value.iter_mut().map(|x| *x * nb);
        for value in self.value.iter_mut() {
            *value *= nb
        }
        self.clean_up();
    }

    fn clean_up(&mut self) {
        let start_length = self.value.len();
        for index in 0.. {
            if *self.value.last().unwrap() < 10 && index >= start_length {
                break;
            }

            let current_value = self.value[index];
            if current_value >= 10 {
                let modulo = current_value % 10;
                let division = current_value / 10;
                self.value[index] = modulo;

                if index == self.value.len() -1 {
                    self.value.push(division);
                } else {
                    self.value[index + 1] += division;
                }
            }
        }
    }
}

pub fn sum_big_numbers(nb1: &BigNumber, nb2: &BigNumber) -> BigNumber {
    let nb1_value = nb1.value();
    let nb2_value = nb2.value();
    let mut sum: Vec<u32> = nb2_value.iter().zip(nb1_value.iter()).map(|(a, b)| a + b).collect();

    if nb1.value().len() > nb2.value().len() {
        sum.extend_from_slice(&nb1_value[nb2_value.len()..]);
    } else {
        sum.extend_from_slice(&nb2_value[nb1_value.len()..]);
    }

    let mut sum = BigNumber::new_from_vec(sum);
    sum.clean_up();
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_big_number_from_nb() {
        let result = BigNumber::new(123);
        assert_eq!(result.value(), &vec![3, 2, 1]);
    }

    #[test]
    fn create_big_number_from_string() {
        let result = BigNumber::new_from_string("254");
        assert_eq!(result.value(), &vec![4, 5, 2]);
    }

    #[test]
    fn create_big_number_from_vec() {
        let result = BigNumber::new_from_vec(vec![7, 6, 3]);
        assert_eq!(result.value(), &vec![7, 6, 3]);
    }
    
    #[test]
    fn test_conversion() {
        let result = BigNumber::new(123).to_number();
        assert_eq!(result, 123);
    }

    #[test]
    fn test_conversion2() {
        let result = BigNumber::new(1230).to_number();
        assert_eq!(result, 1230);
    }

    #[test]
    fn test_sum_different_length() {
        let result = sum_big_numbers(&BigNumber::new(121), &BigNumber::new(7354));
        assert_eq!(result.to_number(), 7475);
    }

    #[test]
    fn test_sum_different_length2() {
        let result = sum_big_numbers(&BigNumber::new_from_vec(vec![2, 1]), &BigNumber::new_from_vec(vec![8]));
        assert_eq!(result.value(), &vec![0, 2]);
    }

    #[test]
    fn test_sum_same_length() {
        let result = sum_big_numbers(&BigNumber::new(342), &BigNumber::new(125));
        assert_eq!(result.to_number(), 467);
    }

    #[test]
    fn test_clean_up() {
        let mut nb = BigNumber::new_from_vec(vec![11, 2, 13]);
        nb.clean_up();
        assert_eq!(nb.value(), &vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_clean_up2() {
        let mut nb = BigNumber::new_from_vec(vec![11, 2, 13, 9, 9]);
        nb.clean_up();
        assert_eq!(nb.value(), &vec![1, 3, 3, 0, 0, 1]);
    }

    #[test]
    fn test_mul() {
        let mut nb = BigNumber::new(1234);
        nb.mul_with_nb(5);
        assert_eq!(nb.value(), &vec![0, 7, 1, 6]);
    }
}