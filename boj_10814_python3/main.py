if __name__ == "__main__":
    member_count = int(input())
    user_db = []

    for _ in range(member_count):
        age, user_name = input().split()
        user_db.append((int(age), user_name))

    user_db.sort(key=lambda x: x[0])

    for i in user_db:
        print(i[0], i[1])
