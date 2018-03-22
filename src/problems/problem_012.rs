pub fn solve(nb_divisors_wanted: u32) -> u32 {
    let mut current_triangular_number = 1;
    let mut current_triangular_number_value = 1;
    
    let mut previous_triangular_number_value;

    loop {
        current_triangular_number += 1;
        previous_triangular_number_value = current_triangular_number_value;
        
        current_triangular_number_value = previous_triangular_number_value + current_triangular_number;
        
        if current_triangular_number_value % 2 != 0 { // On suppose que si c'est impaire, le nb de diviseurs sera toujours moindre
            continue;
        }

        let nb_divisors = get_nb_divisors(current_triangular_number_value);

        if nb_divisors >= nb_divisors_wanted {
            return current_triangular_number_value;
        }
    }
}

/* Ne marche pas pour 1 */
fn get_nb_divisors(number: u32) -> u32 {
    let mut nb_divisors = 0;
    let square_root = (number as f32).sqrt() as u32;
    
    if number % square_root == 0 {
        nb_divisors += 1;
    }

    for i in 1..square_root {
        if number % i == 0 {
            nb_divisors += 2; // i lui même ainsi que n / i
        }
    }

    nb_divisors
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::PreciseTime;

    #[test]
    fn test_correct_result() {
        let start_time = PreciseTime::now();
        let result = solve(500);
        let end_time = PreciseTime::now();
        println!("Problem 12 took {} seconds.", start_time.to(end_time));
        assert_eq!(result, 76576500);
    }
}