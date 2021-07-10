use std::collections::HashSet;
use std::io;

fn main() {
    let mut numbers: HashSet<u32> = HashSet::new();

    for _ in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        numbers.insert(input.trim().parse::<u32>().unwrap() % 42);
    }

    println!("{}", numbers.len());
}
