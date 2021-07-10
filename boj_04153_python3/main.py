while True:
    data = [int(n) for n in input().strip().split()]
    if data == [0, 0, 0]: break
    data.sort()
    a, b, c = data

    print("right" if a**2 + b**2 == c**2 else "wrong")
