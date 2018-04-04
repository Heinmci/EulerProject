pub fn sum_big_numbers(nb1: &Vec<u32>, nb2: &Vec<u32>) -> Vec<u32> {
    let mut sum: Vec<u32> = nb2.iter().zip(nb1.iter()).map(|(a, b)| a + b).collect();
    sum.extend_from_slice(&nb2[nb1.len()..]);

    clean_up_big_number(&mut sum);
    
    sum
}

fn clean_up_big_number(result: &mut Vec<u32>) {
    let start_length = result.len();
    for index in 0.. {
        if *result.last().unwrap() < 10 && index >= start_length {
            break;
        }

        let current_value = result[index];
        if current_value >= 10 {
            let modulo = current_value % 10;
            let division = current_value / 10;
            result[index] = modulo;

            if index == result.len() -1 {
                result.push(division);
            } else {
                result[index + 1] += division;
            }
        }
    }
}

pub fn big_number_to_number(big_number: &Vec<u32>) -> u64 {
    let mut result = 0;
    for (index, value) in big_number.iter().enumerate() {
        result += *value as u64 * 10u64.pow(index as u32);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversion() {
        let result = big_number_to_number(&vec![3, 2, 1]);
        assert_eq!(result, 123);
    }

    #[test]
    fn test_conversion2() {
        let result = big_number_to_number(&vec![0, 3, 2, 1]);
        assert_eq!(result, 1230);
    }

    #[test]
    fn test_sum_different_length() {
        let result = sum_big_numbers(&vec![1, 2, 1], &vec![4, 5, 3, 7]);
        assert_eq!(result, vec![5, 7, 4, 7]);
    }

    #[test]
    fn test_sum_same_length() {
        let result = sum_big_numbers(&vec![1, 2, 1], &vec![4, 5, 3]);
        assert_eq!(result, vec![5, 7, 4]);
    }

    #[test]
    fn test_clean_up() {
        let mut nb = vec![11, 2, 13];
        clean_up_big_number(&mut nb);
        assert_eq!(nb, vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_clean_up2() {
        let mut nb = vec![11, 2, 13, 9, 9];
        clean_up_big_number(&mut nb);
        assert_eq!(nb, vec![1, 3, 3, 0, 0, 1]);
    }
}