import sys


class BOJSet():
    def __init__(self) -> None:
        self.set = set()

    def add(self, x: int) -> None:
        """집합에서 x를 추가한다."""
        self.set.add(x)

    def remove(self, x: int) -> None:
        """집합에서 x를 제거한다."""
        if x in self.set:
            self.set.remove(x)

    def check(self, x: int) -> None:
        """집합에 x가 있으면 1, 없으면 0을 출력한다."""
        print(1 if x in self.set else 0)

    def toggle(self, x: int) -> None:
        """집합에 x가 있으면 제거하고, 없으면 추가한다."""
        if x in self.set:
            self.set.remove(x)
        else:
            self.set.add(x)

    def all(self) -> None:
        """집합을 {1, 2, 3, ..., 20}으로 바꾼다."""
        self.set = set(range(1, 21))

    def empty(self) -> None:
        """집합을 공집합으로 바꾼다."""
        self.set = set()


if __name__ == "__main__":
    boj_set = BOJSet()
    m = int(sys.stdin.readline().strip())

    for _ in range(m):
        command = sys.stdin.readline().strip().split()

        if command[0] == "add":
            boj_set.add(int(command[1]))
        elif command[0] == "remove":
            boj_set.remove(int(command[1]))
        elif command[0] == "check":
            boj_set.check(int(command[1]))
        elif command[0] == "toggle":
            boj_set.toggle(int(command[1]))
        elif command[0] == "all":
            boj_set.all()
        elif command[0] == "empty":
            boj_set.empty()
