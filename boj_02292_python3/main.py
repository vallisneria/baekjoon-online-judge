def honeycomb_dist(room_number: int) -> int:
    """벌집 중앙(1번)에서 지정된 번호의 방으로 가는 가장 짧은 거리를 구하는 함수"""
    step = 1            # 스텝 수
    latest_room_no = 1  # 해당 스텝으로 갈 수 있는 최대 방 번호

    while room_number > latest_room_no:
        latest_room_no += 6 * step
        step += 1

    return step


if __name__ == "__main__":
    input_num = int(input())
    print(honeycomb_dist(input_num))
