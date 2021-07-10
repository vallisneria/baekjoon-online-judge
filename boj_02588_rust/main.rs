fn main() {
    let a = input_number();
    let b = input_number();

    println!("{}", a * (b % 10));
    println!("{}", a * ((b / 10) % 10));
    println!("{}", a * ((b / 100) % 10));
    println!("{}", a * b);
}

fn input_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERROR");
    input.trim().parse().unwrap()
}
