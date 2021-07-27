import sys

if __name__ == "__main__":
    passwords = dict()
    n, m = [int(x) for x in sys.stdin.readline().strip().split()]

    # 사이트 주소와 비밀번호 저장
    for _ in range(n):
        input_data = sys.stdin.readline().strip().split()
        passwords[input_data[0]] = input_data[1]

    # 저장된 사이트 찾기
    for _ in range(m):
        input_data = sys.stdin.readline().strip()
        print(passwords[input_data])
