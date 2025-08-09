text = input().strip().upper()
chars = list(set(text))
char_count = [text.count(char) for char in chars]
max_count = max(char_count)

if char_count.count(max_count) > 1:
    print("?")
else:
    print(chars[char_count.index(max_count)])
