fn main() {
    let count = input_number();

    for i in 1..(count + 1) {
        println!(
            "{star:>width$}",
            star = "*".repeat(i as usize),
            width = count as usize
        );
    }
}

fn input_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}