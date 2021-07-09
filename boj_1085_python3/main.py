x, y, w, h = [int(n) for n in input().strip().split()]
print(min(x, y, w-x, h-y))