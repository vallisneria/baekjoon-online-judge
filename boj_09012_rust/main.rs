use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let epoch: usize = input.trim().parse().unwrap();

    for _ in 0..epoch {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}", is_valid_ps(&(input.trim().to_string())))
    }
}

fn is_valid_ps(ps: &String) -> &str {
    let mut stack: Vec<u8> = Vec::new();

    for char in ps.bytes() {
        if char == 41 && stack.len() >= 1 && stack[stack.len() - 1] == 40 {
            stack.pop();
        } else {
            stack.push(char);
        }
    }

    match stack.len() {
        0 => "YES",
        _ => "NO",
    }
}
