import sys


class BOJQueue():
    """
    boj_10845 <큐>

    정수를 저장하는 큐를 구현한 다음,
    입력으로 주어지는 명령을 처리하는 프로그램을 작성하시오.
    """

    def __init__(self) -> None:
        self.__queue = []

    def push(self, x) -> None:
        """정수 x를 큐에 넣는 연산이다."""
        self.__queue.append(x)

    def pop(self) -> None:
        """큐에서 가장 앞에 있는 정수를 빼고, 그 수를 출력한다.
        만약 큐에 들어있는 정수가 없을 경우에는 -1을 출력한다."""
        if len(self.__queue) >= 1:
            print(self.__queue.pop(0))
        else:
            print("-1")

    def size(self) -> None:
        """큐에 들어있는 정수의 개수를 출력한다."""
        print(len(self.__queue))

    def empty(self) -> None:
        """큐가 비어있으면 1, 아니면 0을 출력한다."""
        print(1 if len(self.__queue) == 0 else 0)

    def front(self) -> None:
        """큐의 가장 앞에 있는 정수를 출력한다.
        만약 큐에 들어있는 정수가 없을 경우에는 -1을 출력한다."""
        print(self.__queue[0] if len(self.__queue) > 0 else -1)

    def back(self) -> None:
        """큐의 가장 뒤에 있는 정수를 출력한다.
        만약 큐에 들어있는 정수가 없을 경우에는 -1을 출력한다."""
        print(self.__queue[-1] if len(self.__queue) > 0 else -1)


if __name__ == "__main__":
    queue = BOJQueue()
    num_of_commands = int(sys.stdin.readline().strip())

    for _ in range(num_of_commands):
        command = sys.stdin.readline().strip().split()

        if command[0] == "push":
            queue.push(int(command[1]))
        elif command[0] == "pop":
            queue.pop()
        elif command[0] == "size":
            queue.size()
        elif command[0] == "empty":
            queue.empty()
        elif command[0] == "front":
            queue.front()
        elif command[0] == "back":
            queue.back()
