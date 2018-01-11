fn main() {
    let solution = get_sum_of_primes_below(2000000);
    println!("Solution: {}", solution);
}

fn get_sum_of_primes_below(limit: u64) -> u64 {
    let mut sum_of_primes = 2;
    let mut counter = 3;
    loop {
        if counter > limit {
            return sum_of_primes;
        }

        if is_prime(counter) {
            sum_of_primes += counter;
        }

        counter += 2;
    }
}

fn is_prime(number: u64) -> bool {
    match number {
        0 | 1 => false,
        2 => true,
        x => {
            for i in  2..((x as f64).sqrt() as u64) + 1 {
                if x % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}