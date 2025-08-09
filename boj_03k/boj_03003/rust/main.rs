use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    println!(
        "{king} {queen} {rook} {bishop} {knight} {pawn}",
        king = 1 - numbers[0],
        queen = 1 - numbers[1],
        rook = 2 - numbers[2],
        bishop = 2 - numbers[3],
        knight = 2 - numbers[4],
        pawn = 8 - numbers[5]
    );
}
