use std::fmt::Display;
use std::io;

/// 정수를 저장하는 덱을 구현한 다음,
/// 입력으로 주어지는 명령을 처리하는 프로그램을 작성하시오.
struct BOJDeque<T>
where
    T: Display + Copy,
{
    deque: Vec<T>,
}

impl<T> BOJDeque<T>
where
    T: Display + Copy,
{
    /// 정수 x를 덱의 앞에 넣는 연산이다.
    fn push_front(&mut self, x: T) {
        self.deque.insert(0, x);
    }

    /// 정수 x를 덱의 뒤에 넣는 연산이다.
    fn push_back(&mut self, x: T) {
        self.deque.push(x);
    }

    /// 덱에서 가장 앞에 있는 정수를 빼고, 그 수를 출력한다.
    /// 만약 덱에 들어있는 정수가 없는 경우에는 -1을 출력한다.
    fn pop_front(&mut self) -> Result<T, ()> {
        if self.deque.len() > 0 {
            let result = self.deque.remove(0);
            println!("{}", result);
            Ok(result)
        } else {
            println!("-1");
            Err(())
        }
    }

    /// 덱의 가장 뒤에 있는 수를 빼고, 그 수를 출력한다.
    /// 만약 덱에 들어 있는 정수가 없을 경우에는 -1을 출력한다.
    fn pop_back(&mut self) -> Result<T, ()> {
        match self.deque.pop() {
            Some(x) => {
                println!("{}", x);
                Ok(x)
            }
            None => {
                println!("-1");
                Err(())
            }
        }
    }

    /// 덱에 들어있는 정수의 개수를 출력한다.
    fn size(&self) -> usize {
        let len = self.deque.len();
        println!("{}", len);
        len
    }

    /// 덱이 비어있으면 1, 아니면 0을 출력한다.
    fn empty(&self) -> bool {
        if self.deque.len() == 0 {
            println!("1");
            true
        } else {
            println!("0");
            false
        }
    }

    /// 덱의 가장 앞에 있는 정수를 출력한다.
    /// 만약 덱에 들어있는 정수가 없는 경우에는 -1을 출력한다.
    fn front(&self) -> Result<T, ()> {
        if self.deque.len() > 0 {
            let first = self.deque[0];
            println!("{}", first);
            Ok(first)
        } else {
            println!("-1");
            Err(())
        }
    }

    /// 덱에 가장 뒤에 있는 정수를 출력한다.
    /// 만약 덱에 들어있는 정수가 없는 경우에는 -1을 출력한다.
    fn back(&self) -> Result<T, ()> {
        if self.deque.len() > 0 {
            let last = self.deque[self.deque.len() - 1];
            println!("{}", last);
            Ok(last)
        } else {
            println!("-1");
            Err(())
        }
    }
}

impl<T> BOJDeque<T>
where
    T: Display + Copy,
{
    fn new() -> BOJDeque<T> {
        BOJDeque {
            deque: Vec::<T>::new(),
        }
    }
}

enum Command {
    PushFront(u32),
    PushBack(u32),
    PopFront,
    PopBack,
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
            "push_front" => Command::PushFront(command[1].parse::<u32>().unwrap()),
            "push_back" => Command::PushBack(command[1].parse::<u32>().unwrap()),
            "pop_front" => Command::PopFront,
            "pop_back" => Command::PopBack,
            "size" => Command::Size,
            "empty" => Command::Empty,
            "front" => Command::Front,
            "back" => Command::Back,
            _ => panic!(),
        }
    }
}

fn main() {
    let mut boj_deque: BOJDeque<u32> = BOJDeque::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_of_test_cases: u32 = input.trim().parse().unwrap();

    for _ in 0..num_of_test_cases {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match Command::interpretation(&input) {
            Command::PushFront(x) => {
                boj_deque.push_front(x);
            }
            Command::PushBack(x) => {
                boj_deque.push_back(x);
            }
            Command::PopFront => {
                boj_deque.pop_front();
            }
            Command::PopBack => {
                boj_deque.pop_back();
            }
            Command::Size => {
                boj_deque.size();
            }
            Command::Empty => {
                boj_deque.empty();
            }
            Command::Front => {
                boj_deque.front();
            }
            Command::Back => {
                boj_deque.back();
            }
        };
    }
}
