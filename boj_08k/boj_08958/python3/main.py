epoch = int(input())

for _ in range(0, epoch):
    quiz = input().strip()
    score = sum([((len(i)+1)*len(i))//2 for i in quiz.split("X") if i != ""])
    print(score)
