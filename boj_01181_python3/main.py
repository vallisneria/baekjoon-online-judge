if __name__ == "__main__":
    word_count = int(input())
    words = set()

    for _ in range(word_count):
        word = input().strip()
        words.add(word)

    words = sorted(words, key=lambda x: (len(x), x))

    for word in words:
        print(word)
