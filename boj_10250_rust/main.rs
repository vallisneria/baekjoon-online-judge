fn main() {
    let count = input_number();

    for _i in 0..count {
        let numbers = input_array();
        println!("{}", solution(&numbers));
    }
}

fn solution(numbers: &Vec<i32>) -> String {
    let floor = ((numbers[2] - 1) % numbers[0]) + 1;
    let room = (numbers[2] as f32 / numbers[0] as f32).ceil();
    format!("{}{:02}", floor, room).to_string()
}

fn input_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
