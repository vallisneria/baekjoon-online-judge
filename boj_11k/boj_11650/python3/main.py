if __name__ == "__main__":
    dot_count = int(input())
    dots = []

    for _ in range(dot_count):
        x, y = [int(x) for x in input().split()]
        dots.append((x, y))

    dots.sort(key=lambda x: (x[0], x[1]))

    for dot in dots:
        print(dot[0], dot[1])
