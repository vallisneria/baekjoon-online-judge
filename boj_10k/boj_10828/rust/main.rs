use std::fmt::Display;
use std::io;

/// 정수를 저장하는 스택을 구현한 다음,
/// 입력으로 주어지는 명령을 처리하는 프로그램을 작성하시오.
struct BOJStack<T>
where
    T: Display + Copy,
{
    stack: Vec<T>,
}

impl<T> BOJStack<T>
where
    T: Display + Copy,
{
    /// 정수 x를 스택에 넣는 연산이다.
    fn push(&mut self, x: T) {
        self.stack.push(x);
    }

    /// 스택에서 가장 위에 있는 정수를 빼고, 그 수를 출력한다.
    /// 만약 스택에 들어있는 정수가 없을 경우에는 -1을 출력한다.
    fn pop(&mut self) -> Result<T, ()> {
        let result = match self.stack.pop() {
            Some(x) => {
                println!("{}", &x);
                Ok(x)
            }
            None => {
                println!("-1");
                Err(())
            }
        };

        result
    }

    fn size(&self) -> usize {
        let size = self.stack.len();
        println!("{}", size);
        size
    }

    fn empty(&self) -> bool {
        match self.stack.len() == 0 {
            true => {
                println!("1");
                true
            }
            false => {
                println!("0");
                false
            }
        }
    }

    fn top(&self) -> Result<T, ()> {
        if self.stack.len() > 0 {
            let result = self.stack[self.stack.len() - 1];
            println!("{}", result);
            Ok(result)
        }else {
            println!("-1");
            Err(())
        }
    }
}

impl<T> BOJStack<T>
where
    T: Display + Copy,
{
    fn new() -> BOJStack<T> {
        BOJStack::<T> {
            stack: Vec::<T>::new(),
        }
    }
}

enum Command {
    Push(u32),
    Pop,
    Size,
    Empty,
    Top,
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
            "top" => Command::Top,
            _ => panic!(),
        }
    }
}

fn main() {
    let mut boj_stack: BOJStack<u32> = BOJStack::<u32>::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_of_test_cases: u32 = input.trim().parse().unwrap();

    for _ in 0..num_of_test_cases {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match Command::interpretation(&input) {
            Command::Push(x) => {
                boj_stack.push(x);
            }
            Command::Pop => {
                boj_stack.pop();
            }
            Command::Size => {
                boj_stack.size();
            }
            Command::Empty => {
                boj_stack.empty();
            }
            Command::Top => {
                boj_stack.top();
            }
        };
    }
}
