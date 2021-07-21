def minimal_constructor(n: int) -> int:
    """
    어떤 자연수 n이 있을 때, 그 자연수 n의 분해합은 n과 n을 이루는 각 자리수의 합을 의미한다.
    어떤 자연수 m의 분해합이 n일 경우, m을 n의 생성자라고 한다.

    이 함수는 자연수 n의 가장 작은 생성자를 찾는 함수이다.
    """
    for i in range(1, n+1):
        decomposition_sum = sum([int(x) for x in str(i)]) + i

        if decomposition_sum == n:
            return i

    return 0


if __name__ == "__main__":
    data = int(input())
    print(minimal_constructor(data))
