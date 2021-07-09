use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();

    for _ in 0..count {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num: usize = input.trim().parse().unwrap();
        println!(
            "{zero} {one}",
            zero = fibo_in_zero(num),
            one = fibo_in_one(num)
        );
    }
}

// fibo(num)을 호출했을 때 fibo(0)이 몇 번 호출되는지 계산하는 함수
fn fibo_in_zero(num: usize) -> usize {
    let mut result: Vec<usize> = vec![1, 0];
    for i in 2..num + 1 {
        result.push(result[i - 1] + result[i - 2]);
    }

    if num <= 1 {
        result[num]
    } else {
        result[result.len() - 1]
    }
}

// fibo(num)을 호출했을 때 fibo(1)이 몇 번 호출되는지 계산하는 함수
fn fibo_in_one(num: usize) -> usize {
    let mut result: Vec<usize> = vec![0, 1];
    for i in 2..num + 1 {
        result.push(result[i - 1] + result[i - 2]);
    }

    if num <= 1 {
        result[num]
    } else {
        result[result.len() - 1]
    }
}
