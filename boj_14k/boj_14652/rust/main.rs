use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    println!(
        "{} {}",
        numbers[2] / numbers[1],
        (numbers[2] % numbers[1])
    );
}
