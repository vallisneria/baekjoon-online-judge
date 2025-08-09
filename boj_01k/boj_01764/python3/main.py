if __name__ == "__main__":
    nolisten = set()
    nosee = set()

    nolisten_count, nosee_count = [int(x) for x in input().split()]

    # 듣도 못한 사람 입력
    for _ in range(nolisten_count):
        nolisten.add(input())

    # 보도 못한 사람 입력
    for _ in range(nosee_count):
        nosee.add(input())

    nolisten_nosee = nolisten & nosee

    print(len(nolisten_nosee))
    print("\n".join(sorted(nolisten_nosee)))
