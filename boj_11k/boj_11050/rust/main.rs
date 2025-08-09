use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", binomial_coefficient(numbers[0], numbers[1]));
}

fn binomial_coefficient(n: u32, k: u32) -> u32 {
    (n - k + 1..=n).product::<u32>() / (1..=k).product::<u32>()
}
