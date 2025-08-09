fn main() {
    loop {
        let numbers = input_array();

        if numbers[0] == 0 && numbers[1] == 0 {
            break;
        }

        println!("{}", solution(numbers[0], numbers[1]))
    }
}

fn solution(a: i32, b: i32) -> i32 {
    a + b
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERROR");
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
