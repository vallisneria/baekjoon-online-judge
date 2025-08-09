def solution(n: int, r: int, c: int) -> int:
    # 4분할 한 것 중 한 칸의 변 길이
    mid = 2 ** (n - 1)

    # 4분할 중 몇번째 칸인지 표시
    quadrant = (2 if mid <= r else 0) + (1 if mid <= c else 0)

    if n == 1:
        return quadrant
    else:
        r = r if quadrant == 0 or quadrant == 1 else r - mid
        c = c if quadrant == 0 or quadrant == 2 else c - mid
        return solution(n - 1, r, c) + (quadrant * (mid ** 2))


if __name__ == "__main__":
    N, r, c = [int(n) for n in input().split()]
    print(solution(N, r, c))
