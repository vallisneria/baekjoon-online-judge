fn main() {
    let input = input_array();

    // 첫째 줄에 (A+B)%C, 둘째 줄에 ((A%C) + (B%C))%C, 셋째 줄에 (A×B)%C, 넷째 줄에 ((A%C) × (B%C))%C를 출력한다.
    println!("{}", (input[0] + input[1]) % input[2]);
    println!("{}", ((input[0] % input[2]) + (input[1] % input[2])) % input[2]);
    println!("{}", (input[0] * input[1]) % input[2]);
    println!("{}", ((input[0] % input[2]) * (input[1] % input[2])) % input[2]);
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERROR");
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
