fn main() {
    let pointx = input_number();
    let pointy = input_number();
    println!("{}", solution(pointx, pointy))
}

fn solution(x: i32, y: i32) -> u8 {
    match (x > 0, y > 0) {
        (true, true) => 1,
        (false, true) => 2,
        (false, false) => 3,
        (true, false) => 4,
    }
}

fn input_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
