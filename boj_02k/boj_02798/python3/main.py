if __name__ == "__main__":
    # 입력 받기
    card_count, blackjack_num = [int(x) for x in input().split()]
    cards = [int(x) for x in input().split()]
    cards_sum = []

    # 합을 전부 계산
    for i in range(card_count):
        for j in range(i+1, card_count):
            for k in range(j+1, card_count):
                sum = cards[i] + cards[j] + cards[k]
                cards_sum += [sum] if sum <= blackjack_num else []

    print(max(cards_sum))
