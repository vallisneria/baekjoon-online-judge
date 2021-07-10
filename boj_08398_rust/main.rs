fn main() {
    let num = input_number();
    println!("{}", solution(num));
}

fn input_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERRER");
    input.trim().parse().unwrap()
}

fn solution(num: i32) -> i32 {
    (1 + num) * num / 2
}
