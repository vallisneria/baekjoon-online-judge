fn main() {
    let num = input_number();
    println!("{}", solution(num));
}

fn solution(score: usize) -> &'static str {
    match score / 10 {
        10 => "A",
        9 => "A",
        8 => "B",
        7 => "C",
        6 => "D",
        _ => "F",
    }
}

fn input_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
