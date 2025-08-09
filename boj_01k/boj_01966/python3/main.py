class NewPrinter():
    def __init__(self, queue: list) -> None:
        self.queue = queue

    def is_head_most_important(self) -> bool:
        """맨 앞에 있는 문서가 가장 중요도가 높은지 확인하는 함수"""
        return self.queue[0] == max(self.queue)

    def move_back(self) -> None:
        """맨 앞에 있는 문서를 맨 뒤로 보내는 함수"""
        doc = self.queue.pop(0)
        self.queue.append(doc)

    def print(self) -> None:
        """맨 앞에 있는 문서를 출력해 버리는 함수"""
        self.queue.pop(0)

    def __len__(self):
        """프린터 큐의 길이를 반환"""
        return len(self.queue)


def solution(m: int, documents: list) -> int:
    result: int = 0
    printer = NewPrinter(documents)

    while m >= 0:
        if printer.is_head_most_important():
            printer.print()
            result += 1
            m -= 1
        else:
            printer.move_back()
            m = m-1 if m != 0 else len(printer) - 1

    return result


if __name__ == "__main__":
    number_of_test_cases = int(input())

    for _ in range(number_of_test_cases):
        n, m = [int(x) for x in input().split()]
        documents = input().split()

        print(solution(m, documents))
