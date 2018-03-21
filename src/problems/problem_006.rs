pub fn solve(limit: u64) -> u64 {
    get_square_of_the_sum(limit) - get_sum_of_squares(limit)
}

fn get_sum_of_squares(limit: u64) -> u64 {
    (1..limit).map(|x| x.pow(2)).sum()
}

fn get_square_of_the_sum(limit: u64) -> u64 {
    (1..limit).sum::<u64>().pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        assert_eq!(solve(101), 25164150);
    }
}
