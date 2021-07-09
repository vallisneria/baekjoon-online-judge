use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse().unwrap();

        if number == 0 {
            break;
        }

        let result = match is_palindrome_number(number) {
            true => "yes",
            false => "no",
        };

        println!("{}", result);
    }
}

fn is_palindrome_number(number: u32) -> bool {
    let mut reversed_number: u32 = 0;

    for digit in 0..=((number as f64).log10().floor() as u32) {
        reversed_number = reversed_number * 10 + (number / u32::pow(10, digit) % 10)
    }

    number == reversed_number
}
