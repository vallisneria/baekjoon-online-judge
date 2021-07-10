use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut numbers: Vec<usize> = input
        .trim()
        .chars()
        .map(|s| s.to_string().parse().unwrap())
        .collect();

    numbers.sort();
    numbers.reverse();

    let result: String = numbers
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{}", result);
}
