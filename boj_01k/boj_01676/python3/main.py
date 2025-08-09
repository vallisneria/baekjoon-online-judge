def five_in_a_factor(x: int) -> int:
    """어떤 수에서 5가 몇 번 곱해져 있는지 구하는 함수이다."""
    result = 0

    while x % 5 == 0:
        x = x // 5
        result += 1

    return result


def count_tail_zero_in_factorial(x: int) -> int:
    """x!의 뒤에서 처음 0이 아닌 숫자가 나올 때까지 나온 0의 개수를 반환하는 함수이다."""
    result = 0

    for i in range(1, 1 + x):
        result += five_in_a_factor(i)

    return result


if __name__ == "__main__":
    num = int(input())
    print(count_tail_zero_in_factorial(num))
