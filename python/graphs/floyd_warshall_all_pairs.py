# Floyd-Warshall All-Pairs Shortest Paths (classic algorithm)
# Given a directed weighted graph (weights may be negative), compute shortest distances
# between every pair of vertices and detect negative cycles.
# Approach: DP over intermediate vertex k: dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]).
# Time: O(V^3)  Space: O(V^2)

INF = float("inf")


def floyd_warshall(n: int, edges: list[tuple[int, int, float]]):
    dist = [[0 if i == j else INF for j in range(n)] for i in range(n)]
    for u, v, w in edges:
        dist[u][v] = min(dist[u][v], w)
    for k in range(n):
        dk = dist[k]
        for i in range(n):
            dik = dist[i][k]
            if dik == INF:
                continue
            di = dist[i]
            for j in range(n):
                if dik + dk[j] < di[j]:
                    di[j] = dik + dk[j]
    has_neg_cycle = any(dist[i][i] < 0 for i in range(n))
    return dist, has_neg_cycle


if __name__ == "__main__":
    d, neg = floyd_warshall(4, [(0, 1, 3), (1, 2, -2), (0, 2, 5), (2, 3, 2)])
    assert d[0][3] == 3 and d[0][2] == 1 and not neg
    _, neg = floyd_warshall(3, [(0, 1, 1), (1, 2, -1), (2, 0, -1)])
    assert neg
    print("dist row 0:", d[0])
