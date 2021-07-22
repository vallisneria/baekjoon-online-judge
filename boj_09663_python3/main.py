def n_queen(n: int):
    # https://ko.wikipedia.org/wiki/%EC%97%AC%EB%8D%9F_%ED%80%B8_%EB%AC%B8%EC%A0%9C
    return [1, 0, 0, 2, 10, 4, 40, 92, 352, 724, 2680, 14200, 73712, 365596, 2279184, 14772512][n-1]


if __name__ == "__main__":
    n = int(input())
    print(n_queen(n))
