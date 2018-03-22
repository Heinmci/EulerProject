pub fn solve(power: u32) -> u32 {
    let mut result: [u8; 1000] = [0; 1000];
    let mut current_array_length: usize = 1;

    result[0] = 2;
    
    for _ in 0..(power - 1) { // -1 parce que on a déjà donné le 2^1 au dessus
        for j in 0..current_array_length {
            result[j] *= 2;
        }
        current_array_length = clean_up_array(&mut result, current_array_length);
    }

    result.iter().enumerate().filter(|&(i, _)| i < current_array_length).fold(0u32,|acc,(_,&b)| acc + b as u32)
}

fn clean_up_array(result: &mut [u8; 1000], mut current_array_length: usize) -> usize{
    for k in 0..current_array_length {
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
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(1000);
        let end_time = PreciseTime::now();
        println!("Problem 16 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 1366);
    }
}
