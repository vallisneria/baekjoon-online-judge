def is_balanced_string(string: str) -> bool:
    """이 문자열이 균형 잡힌 문자열인지 확인하는 함수."""
    stack = []

    for char in string:
        if (char == ")" and len(stack) >= 1 and stack[-1] == "("):
            stack.pop()
        elif (char == "]" and len(stack) >= 1 and stack[-1] == "["):
            stack.pop()
        elif char == "(" or char == "[" or char == ")" or char == "]":
            stack.append(char)

    return len(stack) == 0


if __name__ == "__main__":
    while True:
        string = input()

        if string == ".":
            break

        print("yes" if is_balanced_string(string) else "no")
