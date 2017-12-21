fn main() {
    let sum = sum_of_multiples(1000);
    println!("Sum of multiples of 3 or 5 between 1 and 1000: {}", sum);
}

fn sum_of_multiples(limit: u32) -> u32 {
    (1..limit).filter(|x| x%3 == 0 || x%5 == 0).sum()
}
