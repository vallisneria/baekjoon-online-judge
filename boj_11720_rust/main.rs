use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .trim()
        .split("")
        .filter(|&x| x != "")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", numbers.iter().sum::<u32>());
}
