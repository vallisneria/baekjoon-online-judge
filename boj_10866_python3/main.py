import sys
from typing import Deque


class BOJDeque():
    """
    boj_10866 <덱>

    정수를 저장하는 덱(Deque)을 구현한 다음,
    입력으로 주어지는 명령어를 처리하는 프로그램을 작성하시오.
    """

    def __init__(self) -> None:
        self.__deque = []

    def push_front(self, x, **kwargs) -> None:
        """정수 x를 덱의 앞에 넣는다."""
        self.__deque.insert(0, x)

    def push_back(self, x, **kwargs) -> None:
        """정수 x를 덱의 뒤에 넣는다."""
        self.__deque.append(x)

    def pop_front(self, **kwargs) -> None:
        """덱의 가장 앞에 있는 수를 빼고, 그 수를 출력한다.
        만약 덱에 들어있는 정수가 없는 경우에는 -1을 출력한다."""
        if len(self.__deque) > 0:
            print(self.__deque.pop(0))
        else:
            print(-1)

    def pop_back(self, **kwargs) -> None:
        """덱의 가장 뒤에 있는 수를 빼고, 그 수를 출력한다.
        만약 덱에 들어있는 정수가 없는 경우에는 -1을 출력한다."""
        if len(self.__deque) > 0:
            print(self.__deque.pop())
        else:
            print(-1)

    def size(self, **kwargs) -> None:
        """덱에 들어있는 정수의 개수를 출력한다."""
        print(len(self.__deque))

    def empty(self, **kwargs) -> None:
        """덱이 비어있으면 1, 아니면 0을 출력한다."""
        print(1 if len(self.__deque) == 0 else 0)

    def front(self, **kwargs) -> None:
        """덱의 가장 앞에 있는 정수를 출력한다.
        만약 덱에 들어있는 정수가 없는 경우에는 -1을 출력한다."""
        print(self.__deque[0] if len(self.__deque) > 0 else -1)

    def back(self, **kwargs) -> None:
        """덱의 가장 뒤에 있는 정수를 출력한다.
        만약 덱에 들어있는 정수가 없을 경우에는 -1을 출력한다."""
        print(self.__deque[-1] if len(self.__deque) > 0 else -1)


if __name__ == "__main__":
    deque = BOJDeque()
    num_of_commands = int(sys.stdin.readline().strip())

    for _ in range(num_of_commands):
        command = sys.stdin.readline().strip().split()

        {
            "push_front": deque.push_front,
            "push_back": deque.push_back,
            "pop_front": deque.pop_front,
            "pop_back": deque.pop_back,
            "size": deque.size,
            "empty": deque.empty,
            "front": deque.front,
            "back": deque.back
        }[command[0]](**{"x": command[1] if len(command) > 1 else None})
