use std::io;

fn main() {
    let mut numbers: Vec<u32> = Vec::new();
    let mut result: Vec<u32> = vec![0; 10];

    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        numbers.push(input.trim().parse().unwrap());
    }

    let product: u32 = numbers.iter().product();

    for i in product.to_string().bytes().collect::<Vec<u8>>() {
        result[(i - 0x30) as usize] += 1;
    }

    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
