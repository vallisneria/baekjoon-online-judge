fn main() {
    let option = input_array();
    let numbers = input_array();

    for i in numbers {
        if i < option[1] {
            print!("{} ", i);
        }
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
