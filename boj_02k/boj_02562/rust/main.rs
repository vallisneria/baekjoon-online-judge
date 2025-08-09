use std::io;

fn main() {
    let mut max_num: u8 = 0;
    let mut location: u8 = 0;

    for i in 1..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num = input.trim().parse().unwrap();

        if num > max_num {
            max_num = num;
            location = i;
        }
    }

    println!("{}\n{}", max_num, location);
}
