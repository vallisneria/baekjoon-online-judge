use std::cmp;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let sangsu_1: u32 =
        ((numbers[0] % 10) * 100) + ((numbers[0] / 10 % 10) * 10) + (numbers[0] / 100);
    let sangsu_2: u32 =
        ((numbers[1] % 10) * 100) + ((numbers[1] / 10 % 10) * 10) + (numbers[1] / 100);

    println!("{}", cmp::max(sangsu_1, sangsu_2));
}
