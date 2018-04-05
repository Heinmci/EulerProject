pub mod fibonacci;
pub mod big_number;
pub mod prime;

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

