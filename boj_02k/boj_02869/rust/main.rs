use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let snail: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", snail_climb(snail[0], snail[1], snail[2]));
}

fn snail_climb(a: u32, b: u32, v: u32) -> u32 {
    ((v - a) as f64 / (a - b) as f64).ceil() as u32 + 1
}
