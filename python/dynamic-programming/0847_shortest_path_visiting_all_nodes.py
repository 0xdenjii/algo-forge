# LeetCode 847 - Shortest Path Visiting All Nodes
# Problem: In an undirected connected graph, return the length of the shortest
#   walk that visits every node (start/end anywhere, revisits allowed).
# Approach: Bitmask DP via multi-source BFS over states (node, visited mask);
#   the first state with a full mask gives the answer.
# Time: O(2^n * n^2)  Space: O(2^n * n)
from collections import deque


def shortest_path_length(graph: list[list[int]]) -> int:
    n = len(graph)
    full = (1 << n) - 1
    queue = deque((v, 1 << v, 0) for v in range(n))
    seen = {(v, 1 << v) for v in range(n)}
    while queue:
        v, mask, d = queue.popleft()
        if mask == full:
            return d
        for u in graph[v]:
            state = (u, mask | 1 << u)
            if state not in seen:
                seen.add(state)
                queue.append((*state, d + 1))
    return -1


if __name__ == "__main__":
    assert shortest_path_length([[1, 2, 3], [0], [0], [0]]) == 4
    assert shortest_path_length([[1], [0, 2, 4], [1, 3, 4], [2], [1, 2]]) == 4
    assert shortest_path_length([[]]) == 0
    print("ok")
