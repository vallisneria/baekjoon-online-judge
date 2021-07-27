def ac_lang(arr: list, arr_len: int, code: str) -> str:
    """
    AC는 정수 배열 연산을 하기 위해 만든 언어이다. 이 언어에는 두 가지 함수 R과 D가 있다.

    - R: 배열에 있는 숫자의 순서를 뒤집는다.
    - D: 배열의 첫 번째 숫자를 버린다. 배열이 비어 있는데 사용하면 에러가 발생한다.

    이 함수는 AC언어의 결과를 문자열 형태로 반환한다. 만약 에러가 발생한 경우 `error`를 반환한다.
    """
    is_result_reverse = code.count("R") % 2 == 1
    code_parse = list(map(len, code.split("R")))
    front_pop_count = sum(code_parse[::2])
    back_pop_count = sum(code_parse[1::2])

    if front_pop_count + back_pop_count > arr_len:
        return "error"

    deleted_arr = arr[front_pop_count:len(arr) - back_pop_count]

    if is_result_reverse:
        deleted_arr.reverse()

    return f"[{','.join(deleted_arr)}]"


if __name__ == "__main__":
    num_of_test_cases = int(input())

    for _ in range(num_of_test_cases):
        commands = input()
        arr_len = int(input())
        arr = input().strip("[]").split(",")
        print(ac_lang(arr, arr_len, commands))
