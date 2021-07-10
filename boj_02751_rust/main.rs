use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let epoch: usize = input.trim().parse().unwrap();
    let mut numbers: Vec<isize> = Vec::new();
    for _ in 0..epoch {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let number: isize = input.trim().parse().unwrap();
        numbers.push(number);
    }
    numbers.sort();
    println!(
        "{sort}",
        sort = numbers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
