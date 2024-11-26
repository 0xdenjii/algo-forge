# LeetCode 787 - Cheapest Flights Within K Stops
# Given flights (from, to, price), find the cheapest price from src to dst using
# at most k stops (k+1 edges), or -1.
# Approach: Bellman-Ford limited to k+1 rounds, relaxing from a snapshot each round.
# Time: O(k * E)  Space: O(n)

def find_cheapest_price(n: int, flights: list[list[int]], src: int, dst: int, k: int) -> int:
    INF = float("inf")
    dist = [INF] * n
    dist[src] = 0
    for _ in range(k + 1):
        prev = dist[:]
        for u, v, w in flights:
            if prev[u] + w < dist[v]:
                dist[v] = prev[u] + w
    return -1 if dist[dst] == INF else dist[dst]


if __name__ == "__main__":
    fl = [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]]
    assert find_cheapest_price(4, fl, 0, 3, 1) == 700
    assert find_cheapest_price(3, [[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0) == 500
    print("cheapest:", find_cheapest_price(4, fl, 0, 3, 1))
