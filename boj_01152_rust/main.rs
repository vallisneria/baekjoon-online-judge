use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let words: Vec<&str> = input.trim().split_whitespace().collect();

    println!("{}", words.len());
}
