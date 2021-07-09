use std::cmp;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let epoch: usize = input.trim().parse().unwrap();
    let mut result: Vec<&str> = Vec::new();

    for _ in 0..epoch {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let data: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let d = distance(data[0], data[1], data[3], data[4]);
        let r1 = cmp::max(data[2], data[5]) as f64;
        let r2 = cmp::min(data[2], data[5]) as f64;

        result.push(if d > (r1 + r2) {
            // 외부에서 만나지 않음
            "0"
        } else if d == (r1 + r2) {
            // 외부에서 접함
            "1"
        } else if d > (r1 - r2) && d < (r1 + r2) {
            // 두 교점
            "2"
        } else if d == (r1 - r2) && d != 0f64 {
            // 내부에서 접함
            "1"
        } else if d < (r1 - r2) {
            // 내부에서 만나지 않음
            "0"
        } else {
            // 두 원이 일치함
            "-1"
        });
    }

    println!("{}", result.join("\n"))
}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt()
}
