# Hopcroft-Karp Maximum Bipartite Matching (classic algorithm)
# Given a bipartite graph with left vertices 0..L-1 and right vertices 0..R-1,
# find a maximum matching.
# Approach: alternate BFS layering from free left vertices with DFS along layered augmenting paths.
# Time: O(E * sqrt(V))  Space: O(V + E)

from collections import deque


def hopcroft_karp(L: int, R: int, adj: list[list[int]]) -> int:
    INF = float("inf")
    match_l, match_r = [-1] * L, [-1] * R
    dist = [0] * L

    def bfs():
        q = deque()
        for u in range(L):
            if match_l[u] == -1:
                dist[u] = 0
                q.append(u)
            else:
                dist[u] = INF
        found = False
        while q:
            u = q.popleft()
            for v in adj[u]:
                w = match_r[v]
                if w == -1:
                    found = True
                elif dist[w] == INF:
                    dist[w] = dist[u] + 1
                    q.append(w)
        return found

    def dfs(u):
        for v in adj[u]:
            w = match_r[v]
            if w == -1 or (dist[w] == dist[u] + 1 and dfs(w)):
                match_l[u], match_r[v] = v, u
                return True
        dist[u] = INF
        return False

    matching = 0
    while bfs():
        for u in range(L):
            if match_l[u] == -1 and dfs(u):
                matching += 1
    return matching


if __name__ == "__main__":
    adj = [[0, 1], [0], [1, 2], [2]]
    assert hopcroft_karp(4, 3, adj) == 3
    assert hopcroft_karp(2, 1, [[0], [0]]) == 1
    print("matching:", hopcroft_karp(4, 3, adj))
