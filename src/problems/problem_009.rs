pub fn solve(sum: u32) -> Option<u32> {
    for a in 1..sum {
        for b in (a + 1)..sum {
            for c in (b + 1)..sum {
                if (a + b + c) != sum {
                    continue;
                }
                if is_pythagorean_triplet(a, b, c) {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}

fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;
    
    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(1000).unwrap();
        let end_time = PreciseTime::now();
        println!("Problem 9 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 31875000);
    }
}