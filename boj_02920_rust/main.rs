use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let result: &str = match input.trim() {
        "1 2 3 4 5 6 7 8" => "ascending",
        "8 7 6 5 4 3 2 1" => "descending",
        _ => "mixed",
    };

    println!("{}", result);
}
