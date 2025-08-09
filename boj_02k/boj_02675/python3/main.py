epoch = int(input())

for _ in range(0, epoch):
    repeat, string = input().strip().split(" ")
    string = "".join([char * int(repeat) for char in list(string)])
    print(string)
