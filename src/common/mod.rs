pub mod fibonacci;
pub mod big_number;

pub fn is_prime(number: u64) -> bool {
    match number {
        0 | 1 => false,
        2 | 3 => true,
        x => {
            if x % 2 == 0 || x % 3 == 0 {
                return false;
            } 
            for i in  (5..((x as f64).sqrt() as u64) + 1).step_by(6) {
                if x % i == 0 || x % (i + 2) == 0 {
                    return false;
                }
            }
            true
        }
    }
}

pub fn get_proper_divisor_sum(number: u32) -> u32 {
    let mut sum_divisors = 1;
    let square_root = (number as f32).sqrt() as u32;

    for i in 2..(square_root + 1) {
        if number % i == 0 {
            sum_divisors += i;
            sum_divisors += number / i;
        }
    }

    if square_root * square_root == number {
        sum_divisors -= square_root;
    }

    sum_divisors
}

