fn main() {
    let num = input_number();

    if solution(num) {
        println!("1");
    } else {
        println!("0");
    }
}

fn solution(year: usize) -> bool {
    match (year % 4 == 0, year % 100 == 0, year % 400 == 0) {
        (true, false, _) => true, // 4의 배수이면서, 100의 배수가 아닐 때
        (_, _, true) => true,     // 400의 배수일 때
        _ => false,
    }
}

fn input_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
