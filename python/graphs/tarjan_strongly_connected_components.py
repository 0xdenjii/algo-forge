# Tarjan's Strongly Connected Components (classic algorithm)
# Given a directed graph, partition its vertices into strongly connected components,
# where every vertex in a component can reach every other.
# Approach: single DFS tracking discovery index and low-link with an explicit stack.
# Time: O(V + E)  Space: O(V)

import sys


def tarjan_scc(n: int, adj: list[list[int]]) -> list[list[int]]:
    sys.setrecursionlimit(max(10000, 2 * n + 100))
    index = [-1] * n
    low = [0] * n
    on_stack = [False] * n
    stack, sccs = [], []
    counter = 0

    def dfs(u):
        nonlocal counter
        index[u] = low[u] = counter
        counter += 1
        stack.append(u)
        on_stack[u] = True
        for v in adj[u]:
            if index[v] == -1:
                dfs(v)
                low[u] = min(low[u], low[v])
            elif on_stack[v]:
                low[u] = min(low[u], index[v])
        if low[u] == index[u]:
            comp = []
            while True:
                w = stack.pop()
                on_stack[w] = False
                comp.append(w)
                if w == u:
                    break
            sccs.append(sorted(comp))

    for u in range(n):
        if index[u] == -1:
            dfs(u)
    return sccs


if __name__ == "__main__":
    adj = [[1], [2], [0, 3], [4], [5], [3], []]
    comps = sorted(tarjan_scc(7, adj))
    assert comps == [[0, 1, 2], [3, 4, 5], [6]]
    print("sccs:", comps)
