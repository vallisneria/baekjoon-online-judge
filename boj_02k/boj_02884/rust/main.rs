fn main() {
    let mut time = input_array();

    if time[1] < 45 {
        if time[0] == 0 {
            time[0] = 23;
        } else {
            time[0] -= 1;
        }
        time[1] += 15;
    } else {
        time[1] -= 45;
    }

    println!("{} {}", time[0], time[1]);
}

fn input_array() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("ERROR");
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
