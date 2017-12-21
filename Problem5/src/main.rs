fn main() {
    let result = solve(20);
    println!("Solution: {}", result);
}

fn solve(limit: u8) -> u32 {
    let mut number = 20;
    'outer: loop {
        for i in 1..21 {
            if number % i != 0 {
                number += 1;
                continue 'outer;
            }
        }
        break;
    }
    number
}