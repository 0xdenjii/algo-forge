# Problem: LeetCode 847 - Shortest Path Visiting All Nodes
#
# Return the length of the shortest walk that visits every node of an undirected
# connected graph (n <= 12); nodes and edges may be revisited.
#
# Approach: Multi-source BFS over states (node, visited_mask).
# Complexity: O(n^2 2^n) time, O(n 2^n) space

from collections import deque


def shortest_path_length(graph: list[list[int]]) -> int:
    n = len(graph)
    full = (1 << n) - 1
    q = deque((i, 1 << i, 0) for i in range(n))
    seen = {(i, 1 << i) for i in range(n)}
    while q:
        node, mask, dist = q.popleft()
        if mask == full:
            return dist
        for nb in graph[node]:
            state = (nb, mask | 1 << nb)
            if state not in seen:
                seen.add(state)
                q.append((nb, state[1], dist + 1))
    return -1


if __name__ == "__main__":
    assert shortest_path_length([[1, 2, 3], [0], [0], [0]]) == 4
    assert shortest_path_length([[1], [0, 2, 4], [1, 3, 4], [2], [1, 2]]) == 4
    print("ok")
