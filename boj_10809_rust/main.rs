use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let byte: Vec<u8> = input.trim().bytes().collect();
    let mut result: Vec<isize> = vec![-1; 26];

    for i in 0..byte.len() {
        if result[(byte[i] - 0x61) as usize] == -1 {
            result[(byte[i] - 0x61) as usize] = i as isize;
        }
    }

    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
