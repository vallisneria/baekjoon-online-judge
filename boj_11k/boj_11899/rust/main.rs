use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result = ps_stack(&input.trim().to_string());

    println!("{}", result);
}

fn ps_stack(ps: &String) -> usize {
    let mut stack: Vec<u8> = Vec::new();

    for char in ps.bytes() {
        if char == 41 && stack.len() >= 1 && stack[stack.len() - 1] == 40 {
            stack.pop();
        } else {
            stack.push(char);
        }
    }

    stack.len()
}
