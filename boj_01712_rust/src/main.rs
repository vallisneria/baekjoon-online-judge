fn main() {
    let numbers = input_array();
    println!("{}", solution(&numbers));
}

fn solution(numbers: &Vec<i32>) -> i32 {
    if numbers[2] > numbers[1] {
        //손익분기점: [A / (C - B)] + 1
        (numbers[0] / (numbers[2] - numbers[1])) + 1
    } else {
        // 제작 비용이 판매가보다 크거나 같으면
        // 절대 손익분기점에 도달할 수 없음
        -1
    }
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
