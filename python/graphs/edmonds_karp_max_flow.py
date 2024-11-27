# Edmonds-Karp Maximum Flow (classic algorithm)
# Given a directed graph with edge capacities, a source s and sink t, compute the maximum
# flow from s to t.
# Approach: Ford-Fulkerson using BFS to find shortest augmenting paths in the residual graph.
# Time: O(V * E^2)  Space: O(V^2) with a capacity matrix

from collections import deque


def max_flow(n: int, edges: list[tuple[int, int, int]], s: int, t: int) -> int:
    cap = [[0] * n for _ in range(n)]
    adj = [set() for _ in range(n)]
    for u, v, c in edges:
        cap[u][v] += c
        adj[u].add(v)
        adj[v].add(u)
    flow = 0
    while True:
        parent = [-1] * n
        parent[s] = s
        q = deque([s])
        while q and parent[t] == -1:
            u = q.popleft()
            for v in adj[u]:
                if parent[v] == -1 and cap[u][v] > 0:
                    parent[v] = u
                    q.append(v)
        if parent[t] == -1:
            return flow
        bottleneck, v = float("inf"), t
        while v != s:
            bottleneck = min(bottleneck, cap[parent[v]][v])
            v = parent[v]
        v = t
        while v != s:
            u = parent[v]
            cap[u][v] -= bottleneck
            cap[v][u] += bottleneck
            v = u
        flow += bottleneck


if __name__ == "__main__":
    e = [(0, 1, 16), (0, 2, 13), (1, 2, 10), (2, 1, 4), (1, 3, 12), (3, 2, 9),
         (2, 4, 14), (4, 3, 7), (3, 5, 20), (4, 5, 4)]
    assert max_flow(6, e, 0, 5) == 23
    print("max flow:", max_flow(6, e, 0, 5))
