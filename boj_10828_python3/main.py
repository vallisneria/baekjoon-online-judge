import sys


class BOJStack():
    """
    정수를 저장하는 스택을 구현한 다음, 입력으로 주어지는 명령을 처리하는 프로그램을 작성하시오.
    """

    def __init__(self) -> None:
        self.stack = []

    def push(self, x: int) -> None:
        """정수 X를 스택에 넣는다."""
        self.stack.append(x)

    def pop(self) -> None:
        """
        스택에서 가장 위에 있는 정수를 빼고, 그 수를 출력한다.
        만약 스택에 들어있는 정수가 없을 경우 -1을 출력한다.
        """
        if len(self.stack) > 0:
            latest = self.stack.pop()
        else:
            latest = -1

        print(latest)

    def size(self) -> None:
        """스택에 들어있는 정수의 개수를 출력한다."""
        print(len(self.stack))

    def empty(self) -> None:
        """스택이 비어있으면 1, 아니면 0을 출력한다."""
        print(0 if len(self.stack) > 0 else 1)

    def top(self) -> None:
        """
        스택의 가장 위에 있는 정수를 출력한다.
        만약 스택에 들어 있는 정수가 없는 경우에는 -1을 출력한다.
        """
        if len(self.stack) > 0:
            latest = self.stack[-1]
        else:
            latest = -1

        print(latest)


if __name__ == "__main__":
    epoch = int(sys.stdin.readline().strip())
    stack = BOJStack()

    for _ in range(epoch):
        command = sys.stdin.readline().strip().split()

        if command[0] == "push":
            stack.push(command[1])
        elif command[0] == "pop":
            stack.pop()
        elif command[0] == "size":
            stack.size()
        elif command[0] == "empty":
            stack.empty()
        elif command[0] == "top":
            stack.top()
