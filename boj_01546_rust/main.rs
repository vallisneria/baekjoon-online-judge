use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let scores: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let max_score = scores.iter().max().unwrap();
    let new_score: f32 = scores
        .iter()
        .map(|x| (*x as f32) / (*max_score as f32) * 100f32)
        .sum();

    println!("{}", new_score / count as f32)
}
