fn main() {
    let sum = sum_fibo(4000000);
    println!("Sum: {}", sum);
}

fn sum_fibo(limit: u32) -> u32 {
    let mut sum = 2;
    let mut nb_1 = 1;
    let mut nb_2 = 2;
    loop {
        let next_number = nb_1 + nb_2;
        if next_number > limit {
            break;
        }
        nb_1 = nb_2;
        nb_2 = next_number;
        if next_number%2 == 0 {
            sum += next_number;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        assert_eq!(sum_fibo(4000000), 4613732);
    }
}