def is_big(a, b) -> bool:
    """
    a와 b의 덩치를 비교하는 함수.
    a가 더 클 경우 True를 반환. 더 작거나 비교할 수 없을 경우 False.
    """
    return (a[0] > b[0]) and (a[1] > b[1])


if __name__ == "__main__":
    count = int(input())
    body_info = []
    result = []

    # 입력 받기
    for _ in range(count):
        x, y = [int(x) for x in input().split()]
        body_info.append((x, y))

    # 비교하기
    for i in body_info:
        rank = 1
        for j in body_info:
            rank += 1 if is_big(j, i) else 0
        result.append(rank)

    print(" ".join([str(x) for x in result]))
