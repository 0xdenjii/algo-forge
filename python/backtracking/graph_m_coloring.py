"""
Graph m-Coloring (classic)
Problem: Given an undirected graph and m colors, assign a color to every
vertex so adjacent vertices differ; return one coloring or None. Also derive
the chromatic number by trying m = 1, 2, ...
Approach: Color vertices in order of decreasing degree; for each vertex try
colors not used by already-colored neighbours.
Time: O(m^V)  Space: O(V)
"""
from typing import Dict, List, Optional, Tuple


def m_coloring(n: int, edges: List[Tuple[int, int]], m: int) -> Optional[List[int]]:
    adj: Dict[int, List[int]] = {v: [] for v in range(n)}
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    order = sorted(range(n), key=lambda v: -len(adj[v]))
    color = [-1] * n

    def dfs(i: int) -> bool:
        if i == n:
            return True
        v = order[i]
        banned = {color[u] for u in adj[v]}
        for c in range(m):
            if c not in banned:
                color[v] = c
                if dfs(i + 1):
                    return True
        color[v] = -1
        return False

    return color if dfs(0) else None


def chromatic_number(n: int, edges: List[Tuple[int, int]]) -> int:
    if n == 0:
        return 0
    return next(m for m in range(1, n + 1) if m_coloring(n, edges, m) is not None)


if __name__ == "__main__":
    petersen = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (0, 5), (1, 6), (2, 7), (3, 8), (4, 9),
                (5, 7), (7, 9), (9, 6), (6, 8), (8, 5)]
    assert m_coloring(10, petersen, 2) is None
    col = m_coloring(10, petersen, 3)
    assert col is not None and all(col[a] != col[b] for a, b in petersen)
    k4 = [(a, b) for a in range(4) for b in range(a + 1, 4)]
    assert chromatic_number(4, k4) == 4
    print(col)
