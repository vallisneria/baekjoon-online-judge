use std::fmt::Display;
use std::io;

/// 정수를 저장하는 큐를 구현한 다음,
/// 입력으로 주어지는 명령을 처리하는 프로그램을 작성하시오.
struct BOJQueue<T>
where
    T: Display + Copy,
{
    queue: Vec<T>,
}

impl<T> BOJQueue<T>
where
    T: Display + Copy,
{
    /// 정수 x를 큐에 넣는 연산이다.
    fn push(&mut self, x: T) {
        self.queue.push(x);
    }

    /// 큐에서 가장 앞에 있는 정수를 빼고, 그 수를 출력한다.
    /// 만약 큐에 들어있는 정수가 없는 경우에는 -1을 출력한다.
    fn pop(&mut self) -> Result<T, ()> {
        if self.queue.len() > 0 {
            let result = self.queue.remove(0);
            println!("{}", result);
            Ok(result)
        } else {
            println!("-1");
            Err(())
        }
    }

    /// 큐에 들어있는 정수의 개수를 출력한다.
    fn size(&self) -> usize {
        let len = self.queue.len();
        println!("{}", len);
        len
    }

    /// 큐가 비어있으면 1, 아니면 0을 출력한다.
    fn empty(&self) -> bool {
        if self.queue.len() == 0 {
            println!("1");
            true
        } else {
            println!("0");
            false
        }
    }

    /// 큐의 가장 앞에 있는 정수를 출력한다.
    /// 만약 큐에 들어있는 정수가 없는 경우에는 -1을 출력한다.
    fn front(&self) -> Result<T, ()> {
        if self.queue.len() > 0 {
            let first = self.queue[0];
            println!("{}", first);
            Ok(first)
        } else {
            println!("-1");
            Err(())
        }
    }

    /// 큐에 가장 뒤에 있는 정수를 출력한다.
    /// 만약 큐에 들어있는 정수가 없는 경우에는 -1을 출력한다.
    fn back(&self) -> Result<T, ()> {
        if self.queue.len() > 0 {
            let last = self.queue[self.queue.len() - 1];
            println!("{}", last);
            Ok(last)
        } else {
            println!("-1");
            Err(())
        }
    }
}

impl<T> BOJQueue<T>
where
    T: Display + Copy,
{
    fn new() -> BOJQueue<T> {
        BOJQueue {
            queue: Vec::<T>::new(),
        }
    }
}

enum Command {
    Push(u32),
    Pop,
    Size,
    Empty,
    Front,
    Back,
}

impl Command {
    /// 문자열 형태로 들어온 명령어를 해석하는 코드
    fn interpretation(str: &String) -> Command {
        let command = str.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();

        match command[0] {
            "push" => Command::Push(command[1].parse::<u32>().unwrap()),
            "pop" => Command::Pop,
            "size" => Command::Size,
            "empty" => Command::Empty,
            "front" => Command::Front,
            "back" => Command::Back,
            _ => panic!(),
        }
    }
}

fn main() {
    let mut boj_queue: BOJQueue<u32> = BOJQueue::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_of_test_cases: u32 = input.trim().parse().unwrap();

    for _ in 0..num_of_test_cases {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match Command::interpretation(&input) {
            Command::Push(x) => {
                boj_queue.push(x);
            }
            Command::Pop => {
                boj_queue.pop();
            }
            Command::Size => {
                boj_queue.size();
            }
            Command::Empty => {
                boj_queue.empty();
            }
            Command::Front => {
                boj_queue.front();
            }
            Command::Back => {
                boj_queue.back();
            }
        };
    }
}
