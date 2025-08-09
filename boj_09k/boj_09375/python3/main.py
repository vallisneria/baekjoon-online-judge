def number_of_outfit_combinations(type_and_number_of_clothes: dict) -> int:
    """옷을 입는 경우의 수를 구하는 함수"""
    result = 1
    for i in type_and_number_of_clothes.values():
        result *= (i + 1)
    
    return result - 1


if __name__ == "__main__":
    num_of_test_case = int(input())

    for _ in range(num_of_test_case):
        num_of_clothes = int(input())
        clothes = dict()

        for _ in range(num_of_clothes):
            _, clothes_class = input().split()

            if clothes_class in clothes:
                clothes[clothes_class] += 1
            else:
                clothes[clothes_class] = 1
        
        print(number_of_outfit_combinations(clothes))
