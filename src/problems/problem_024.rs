pub fn solve(permutation_nb: u32) -> u32 {
    let dictionnary = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result = 0;
    let length = dictionnary.len();
    let mut permutations = vec![];

    for i in (1..length).rev() {
        permutations.push(factorial(i));
    }
    let nb_array = get_permutation(permutation_nb, permutations, dictionnary);
    
    for i in 0..length {
        result += (nb_array[i]) as u32 * 10u32.pow((length - 1) as u32 - i as u32);
    }
    result
    
}

fn get_permutation(mut permutation_nb: u32, permutations: Vec<u32>, mut dictionnary: Vec<u32>) -> Vec<u32> {
    let mut result = vec![];
    for nb in permutations {
        let quotient  = permutation_nb / nb;
        let mut remainder = permutation_nb % nb;
        let value = if remainder == 0 {remainder += nb; quotient - 1} else {quotient};
        result.push(dictionnary[value as usize]);
        dictionnary.remove(value as usize);
        permutation_nb = remainder;
    }
    result.push(dictionnary[0]);
    result
}

fn factorial(nb: usize) -> u32 {
    let mut result = 1;
    for i in 2..nb + 1 {
        result *= i;
    }
    result as u32
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
        println!("Problem 24 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 2783915460);
    }
}
