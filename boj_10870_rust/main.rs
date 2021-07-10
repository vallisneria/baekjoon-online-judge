fn main() {
    let num = input_number();

    println!("{}", fibonacci(num))
}

fn fibonacci(num: usize) -> usize {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}

fn input_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
