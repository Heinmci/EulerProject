pub fn solve(limit: u32) -> u32 {
    let mut max_chain_size = 0;
    let mut number_of_interest = 0;

    for i in 1..limit {
        let chain_size = get_chain_size(i);
        if chain_size > max_chain_size {
            max_chain_size = chain_size;
            number_of_interest = i;
        }
    }

    number_of_interest
}

fn get_chain_size(starting_number: u32) -> u32 {
    let mut chain_size = 1;
    let mut number: u64 = starting_number as u64;

    loop {
        if number == 1 {
            break;
        }

        if number % 2 == 0 {
            number /=  2;
        } else {
            number = number * 3 + 1;
        }
        chain_size += 1;
    }

    chain_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(1_000_000);
        let end_time = PreciseTime::now();
        println!("Problem 14 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 837799);
    }
}
