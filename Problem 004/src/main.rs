fn main() {
    let biggest_palyndrome_product = get_biggest_palyndrome_product(999);
    println!("Biggest palyndrome product: {}", biggest_palyndrome_product);
}

fn get_biggest_palyndrome_product(limit1: u32) -> u32 {
    let mut max = 0;
    for i in (0..limit1 + 1).rev() {
        for j in (0..i + 1).rev() {
            let mult = i * j;
            if  mult > max && is_palydrome(mult) {
                max = mult;
            }
        }
    }
    max
}

fn is_palydrome(number: u32) -> bool {
    let number_str = number.to_string();
    number_str == number_str.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        assert_eq!(get_biggest_palyndrome_product(999), 906609);
    }
}