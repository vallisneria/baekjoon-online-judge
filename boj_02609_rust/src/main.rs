use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!(
        "{}\n{}",
        greatest_common_factor(numbers[0], numbers[1]),
        least_common_multiple(numbers[0], numbers[1])
    );
}

fn greatest_common_factor(a: u32, b: u32) -> u32 {
    match b {
        0 => a,
        _ => greatest_common_factor(b, a % b),
    }
}

fn least_common_multiple(a: u32, b: u32) -> u32 {
    a * b / greatest_common_factor(a, b)
}
