use std::io;

fn main() {
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    println!("{}??!", &id.trim());
}
