# LeetCode 785 - Is Graph Bipartite?
# Given an adjacency list graph[i], return True if nodes can be split into two sets
# such that every edge connects nodes from different sets.
# Approach: BFS 2-coloring over every component; a same-color edge means not bipartite.
# Time: O(V + E)  Space: O(V)

from collections import deque


def is_bipartite(graph: list[list[int]]) -> bool:
    color = [-1] * len(graph)
    for s in range(len(graph)):
        if color[s] != -1:
            continue
        color[s] = 0
        q = deque([s])
        while q:
            u = q.popleft()
            for v in graph[u]:
                if color[v] == -1:
                    color[v] = color[u] ^ 1
                    q.append(v)
                elif color[v] == color[u]:
                    return False
    return True


if __name__ == "__main__":
    assert not is_bipartite([[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]])
    assert is_bipartite([[1, 3], [0, 2], [1, 3], [0, 2]])
    print("bipartite:", is_bipartite([[1, 3], [0, 2], [1, 3], [0, 2]]))
