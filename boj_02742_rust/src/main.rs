use std::io::Write;

fn main() {
    let count = input_number();
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    for i in (1..(count + 1)).rev() {
        writeln!(out, "{}", i).unwrap();
    }
}

fn input_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
