use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let product_code: Vec<u8> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let verification: u8 = product_code.iter().map(|x| x.pow(2)).sum::<u8>() % 10;
    println!("{}", verification);
}
