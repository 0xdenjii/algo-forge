# LeetCode 802 - Find Eventual Safe States
# A node is safe if every path starting from it ends at a terminal node (no outgoing edges).
# Return all safe nodes in ascending order.
# Approach: DFS with colors (0 unvisited, 1 visiting, 2 safe); a node on a cycle is unsafe.
# Time: O(V + E)  Space: O(V)

import sys


def eventual_safe_nodes(graph: list[list[int]]) -> list[int]:
    sys.setrecursionlimit(max(10000, 2 * len(graph) + 100))
    color = [0] * len(graph)

    def safe(u):
        if color[u]:
            return color[u] == 2
        color[u] = 1
        if any(not safe(v) for v in graph[u]):
            return False
        color[u] = 2
        return True

    return [u for u in range(len(graph)) if safe(u)]


if __name__ == "__main__":
    assert eventual_safe_nodes([[1, 2], [2, 3], [5], [0], [5], [], []]) == [2, 4, 5, 6]
    print("safe:", eventual_safe_nodes([[1, 2], [2, 3], [5], [0], [5], [], []]))
