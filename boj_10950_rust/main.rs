fn main() {
    let count = input_number();

    for _i in 0..count {
        let input = input_array();
        println!("{}", input[0] + input[1]);
    }
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERROR");
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn input_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERRER");
    input.trim().parse().unwrap()
}
