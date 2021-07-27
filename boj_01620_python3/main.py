import sys


class Pokedex():
    """
    오박사 : 그럼 다솜아 이제 진정한 포켓몬 마스터가 되기 위해 도감을 완성시키도록 하여라.
    일단 네가 현재 가지고 있는 포켓몬 도감에서 포켓몬의 이름을 보면 포켓몬의 번호를 말하거나,
    포켓몬의 번호를 보면 포켓몬의 이름을 말하는 연습을 하도록 하여라.

    나의 시험을 통과하면, 내가 새로 만든 도감을 주도록 하겠네.
    """

    def __init__(self) -> None:
        self.__id_to_name_list = []
        self.__name_to_id_list = dict()

    def add_pokemon(self, name: str) -> None:
        """도감에 포켓몬을 추가하는 함수"""
        self.__id_to_name_list.append(name)
        self.__name_to_id_list.update({name: len(self.__id_to_name_list)})

    def search_by_name(self, name: str) -> int:
        """이름을 통해 포켓몬 ID를 찾는 함수"""
        return self.__name_to_id_list[name]

    def search_by_id(self, id: int) -> str:
        """ID를 통해 포켓몬 이름을 찾는 함수"""
        return self.__id_to_name_list[id - 1]


if __name__ == "__main__":
    pokedex_count, question_count = [int(x) for x in input().split()]
    pokedex = Pokedex()

    # 도감에 포켓몬을 추가
    for _ in range(pokedex_count):
        pokedex.add_pokemon(sys.stdin.readline().strip())

    # 문제를 받고, 답을 말함
    for _ in range(question_count):
        question = sys.stdin.readline().strip()

        if question.isdigit():
            print(pokedex.search_by_id(int(question)))
        else:
            print(pokedex.search_by_name(question))
