use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("입력한 값을 읽지 못했습니다.");
    let input_number: u8 = input
        .trim()
        .parse()
        .expect("입력한 값이 올바른 숫자가 아닙니다.");

    let mut i: u8 = 1;
    while i <= 9 {
        println!("{} * {} = {}", input_number, i, input_number * i);
        i = i + 1;
    }
}
