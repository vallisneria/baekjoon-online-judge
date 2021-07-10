fn main() {
    let input = input_array();
    println!("{}", input[0] as f64 / input[1] as f64);
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERROR");
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
