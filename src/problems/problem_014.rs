use rayon::prelude::*;

pub fn solve(limit: u32) -> u32 {
    let (_, index) = (1..limit).into_par_iter().enumerate().map(|(index, value)| (get_chain_size(value), index)).max().unwrap();
    index as u32 + 1

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
