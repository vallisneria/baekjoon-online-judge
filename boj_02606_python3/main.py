class SimpleGraph():
    def __init__(self, vertex_count: int) -> None:
        self.graph = [
            set() for row in range(vertex_count)
        ]

    def add_edge(self, start, end) -> None:
        """양방향 간선을 추가합니다."""
        self.graph[start].add(end)
        self.graph[end].add(start)

    def connected_vertex(self, start_vertex: int) -> set:
        """한 정점과 직/간접적으로 연결된 다른 정점의 목록을 가져옵니다."""
        reserve = self.graph[start_vertex]
        not_visited = set(range(len(self.graph)))
        visited = set([])

        while len(reserve) != 0:
            # 예약된 정점들 중 하나 뽑기
            visit = reserve.pop()

            # 뽑은 정점과 이어진 정점 가져오기
            adjacent = set(self.graph[visit])

            # 이어진 정점 중 이미 방문한 정점 제거
            adjacent = adjacent - visited

            # 이어진 정점들을 방문할 정점에 추가하기
            reserve.update(adjacent)

            # 방문한 정점에 추가하기
            visited.add(visit)

            # 방문하지 않은 정점에서 제거하기
            not_visited.remove(visit)

        return visited


if __name__ == "__main__":
    computer_count = int(input())
    edge_count = int(input())
    graph = SimpleGraph(computer_count)

    for _ in range(edge_count):
        start, end = [int(x) for x in input().split()]
        graph.add_edge(start - 1, end - 1)

    print(len(graph.connected_vertex(0)) - 1)
