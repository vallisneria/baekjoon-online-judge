fn main() {
    let num = input_array();

    if num[0] > num[1] {
        println!(">");
    } else if num[0] < num[1] {
        println!("<");
    } else {
        println!("==");
    }
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERROR");
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
