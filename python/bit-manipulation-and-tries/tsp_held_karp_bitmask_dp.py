# Problem: Travelling Salesman (Held-Karp bitmask DP)
#
# Given an n x n distance matrix (n <= ~20), return the minimum cost of a tour
# starting and ending at city 0 that visits every city exactly once.
#
# Approach: dp[mask][j] = min cost to start at 0, visit set mask, end at j.
# Complexity: O(n^2 2^n) time, O(n 2^n) space

from math import inf


def tsp(dist: list[list[int]]) -> float:
    n = len(dist)
    if n == 1:
        return 0
    dp = [[inf] * n for _ in range(1 << n)]
    dp[1][0] = 0
    for mask in range(1 << n):
        if not mask & 1:
            continue
        for j in range(n):
            cur = dp[mask][j]
            if cur == inf or not mask >> j & 1:
                continue
            for k in range(n):
                if mask >> k & 1:
                    continue
                nm = mask | 1 << k
                if cur + dist[j][k] < dp[nm][k]:
                    dp[nm][k] = cur + dist[j][k]
    full = (1 << n) - 1
    return min(dp[full][j] + dist[j][0] for j in range(1, n))


if __name__ == "__main__":
    d = [[0, 10, 15, 20], [10, 0, 35, 25], [15, 35, 0, 30], [20, 25, 30, 0]]
    assert tsp(d) == 80
    print("ok")
