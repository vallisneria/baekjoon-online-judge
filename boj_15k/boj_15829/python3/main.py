def hash(s: str, r: int, m: int) -> int:
    s = [ord(c) - 96 for c in s]

    for i in range(len(s)):
        s[i] = (s[i] * (r ** i)) % m

    return sum(s) % m


if __name__ == "__main__":
    length = int(input())
    input_string = input().strip()

    print(hash(input_string, 31, 1234567891))
