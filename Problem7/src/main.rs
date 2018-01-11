fn main() {
    let solution = get_nth_prime(10001);
    println!("Solution: {}", solution);
}

fn get_nth_prime(nb: u32) -> u64 {
    let mut counter = 5;
    let mut nb_primes = 2;
    loop {
        if is_prime(counter) {
            nb_primes += 1;
            if nb_primes == nb {
                return counter;
            }
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
