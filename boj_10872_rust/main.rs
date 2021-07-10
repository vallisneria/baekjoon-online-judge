fn main() {
    let num = input_number();

    println!("{}", factorial(num))
}

fn factorial(num: usize) -> usize {
    match num {
        0 => 1,
        1 => 1,
        _ => num * factorial(num - 1),
    }
}

fn input_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
