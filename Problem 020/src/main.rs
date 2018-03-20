fn main() {
    let solution = get_sum_of_digits(100);
    println!("Solution: {}", solution);
}

fn get_sum_of_digits(factorial: u64) -> u64 {
    let mut result: [u64; 1000] = [0; 1000];
    let mut current_array_length: usize = 1;

    result[0] = 1;
    
    for nb in 1..(factorial + 1) {
        for j in 0..current_array_length {
            result[j] *= nb;
        }
        current_array_length = clean_up_array(&mut result, current_array_length);
    }

    result.iter().enumerate().filter(|&(i, _)| i < current_array_length).fold(0u64,|acc,(_,&b)| acc + b as u64)
}

fn clean_up_array(result: &mut [u64; 1000], mut current_array_length: usize) -> usize{
    let start_array_length = current_array_length;
    for k in 0..1000 {
        if current_array_length > start_array_length && result[k] == 0 {
            break;
        }
        let current_value = result[k];
        if current_value >= 10 {
            let modulo = current_value % 10;
            let division = current_value / 10;
            result[k] = modulo;
            result[k + 1] += division;
            if k + 1 >= current_array_length {
                current_array_length += 1;
            }
        }
    }
    current_array_length
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        assert_eq!(get_sum_of_digits(100), 648);
    }
}
