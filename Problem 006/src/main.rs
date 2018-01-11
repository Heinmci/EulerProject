fn main() {
    println!("Hello, world!");
    let solution = get_square_of_the_sum(101) - get_sum_of_squares(101);
    println!("Solution: {}", solution);
}

fn get_sum_of_squares(limit: u64) -> u64 {
    (1..limit).map(|x| x.pow(2)).sum()
}

fn get_square_of_the_sum(limit: u64) -> u64 {
    (1..limit).sum::<u64>().pow(2)
}
