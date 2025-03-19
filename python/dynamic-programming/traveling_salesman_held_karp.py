# Classic - Travelling Salesman Problem (Held-Karp bitmask DP)
# Problem: Given a complete distance matrix for n cities, find the minimum cost
#   tour that starts at city 0, visits every city once and returns to 0.
# Approach: dp[mask][v] = min cost to start at 0, visit exactly `mask`, end at v;
#   transition by adding an unvisited city. Close the tour at the end.
# Time: O(2^n * n^2)  Space: O(2^n * n)
from math import inf


def tsp(dist: list[list[int]]) -> int:
    n = len(dist)
    if n == 1:
        return 0
    size = 1 << n
    dp = [[inf] * n for _ in range(size)]
    dp[1][0] = 0
    for mask in range(1, size, 2):  # only masks containing city 0
        row = dp[mask]
        for v in range(n):
            cost = row[v]
            if cost == inf:
                continue
            for u in range(n):
                if mask >> u & 1:
                    continue
                nm = mask | 1 << u
                if cost + dist[v][u] < dp[nm][u]:
                    dp[nm][u] = cost + dist[v][u]
    full = size - 1
    return int(min(dp[full][v] + dist[v][0] for v in range(1, n)))


if __name__ == "__main__":
    d = [[0, 10, 15, 20], [10, 0, 35, 25], [15, 35, 0, 30], [20, 25, 30, 0]]
    assert tsp(d) == 80
    assert tsp([[0, 3], [3, 0]]) == 6
    print("ok")
