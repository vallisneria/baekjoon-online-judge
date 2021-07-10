use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let epoch: usize = input.trim().parse().unwrap();
    for _ in 0..epoch {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        println!("{}", solution(arr[0], arr[1]));
    }
}

fn solution(a: usize, b: usize) -> usize {
    match a % 10 {
        0 => 10,
        1 => 1,
        2 => [2, 4, 8, 6][(b - 1) % 4],
        3 => [3, 9, 7, 1][(b - 1) % 4],
        4 => [4, 6][(b - 1) % 2],
        5 => 5,
        6 => 6,
        7 => [7, 9, 3, 1][(b - 1) % 4],
        8 => [8, 4, 2, 6][(b - 1) % 4],
        9 => [9, 1][(b - 1) % 2],
        _ => panic!(),
    }
}
