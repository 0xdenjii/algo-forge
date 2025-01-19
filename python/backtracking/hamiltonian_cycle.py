"""
Hamiltonian Cycle (classic)
Problem: Given an undirected graph, find a cycle that visits every vertex
exactly once and returns to the start, or report that none exists.
Approach: Fix vertex 0 as the start; extend the path with unvisited
neighbours, backtracking on dead ends; close the cycle if the last vertex is
adjacent to 0.
Time: O(V!)  Space: O(V)
"""
from typing import List, Optional, Set, Tuple


def hamiltonian_cycle(n: int, edges: List[Tuple[int, int]]) -> Optional[List[int]]:
    if n == 0:
        return None
    adj: List[Set[int]] = [set() for _ in range(n)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    path = [0]
    visited = [False] * n
    visited[0] = True

    def dfs() -> bool:
        if len(path) == n:
            return 0 in adj[path[-1]]
        for v in sorted(adj[path[-1]]):
            if not visited[v]:
                visited[v] = True
                path.append(v)
                if dfs():
                    return True
                path.pop()
                visited[v] = False
        return False

    return path + [0] if dfs() else None


if __name__ == "__main__":
    g1 = [(0, 1), (0, 3), (1, 2), (1, 3), (1, 4), (2, 4), (3, 4)]
    cyc = hamiltonian_cycle(5, g1)
    assert cyc is not None and len(set(cyc)) == 5 and cyc[0] == cyc[-1] == 0
    g2 = [(0, 1), (0, 3), (1, 2), (1, 3), (1, 4), (2, 4)]
    assert hamiltonian_cycle(5, g2) is None
    print(cyc)
