use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();
    let mut stack: Vec<usize> = Vec::new();

    for _ in 0..count {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let money: usize = input.trim().parse().unwrap();

        if money == 0 {
            stack.pop();
        } else {
            stack.push(money);
        }
    }

    println!("{}", stack.iter().sum::<usize>())
}
