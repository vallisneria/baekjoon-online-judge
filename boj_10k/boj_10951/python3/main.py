while True:
    try:
        a, b = [int(n) for n in input().strip().split()]
        print(a + b)
    except:
        break
