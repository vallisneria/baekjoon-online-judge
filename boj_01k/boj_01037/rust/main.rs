use std::io;

fn main() {
    // 진짜 약수의 개수를 입력
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let length: usize = input.trim().parse::<usize>().unwrap();

    // 진짜 약수의 목록을 입력
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut real_factors: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    real_factors.sort();

    // 출력
    println!("{}", real_factors[0] * real_factors[length - 1]);
}
