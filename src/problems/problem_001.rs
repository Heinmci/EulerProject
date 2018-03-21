pub fn solve(limit: u32) -> u32 {
    (1..limit).filter(|x| x%3 == 0 || x%5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_result() {
        assert_eq!(solve(1000), 233168);
    }
}
