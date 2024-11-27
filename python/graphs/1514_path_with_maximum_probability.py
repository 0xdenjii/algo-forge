# LeetCode 1514 - Path with Maximum Probability
# Given an undirected graph where each edge has a success probability, return the maximum
# probability of reaching end from start, or 0.
# Approach: Dijkstra on products using a max-heap (negated probabilities).
# Time: O(E log V)  Space: O(V + E)

import heapq


def max_probability(n, edges, succ_prob, start, end) -> float:
    adj = [[] for _ in range(n)]
    for (a, b), p in zip(edges, succ_prob):
        adj[a].append((b, p))
        adj[b].append((a, p))
    best = [0.0] * n
    best[start] = 1.0
    pq = [(-1.0, start)]
    while pq:
        p, u = heapq.heappop(pq)
        p = -p
        if u == end:
            return p
        if p < best[u]:
            continue
        for v, w in adj[u]:
            if p * w > best[v]:
                best[v] = p * w
                heapq.heappush(pq, (-best[v], v))
    return 0.0


if __name__ == "__main__":
    assert abs(max_probability(3, [[0, 1], [1, 2], [0, 2]], [0.5, 0.5, 0.2], 0, 2) - 0.25) < 1e-9
    assert max_probability(3, [[0, 1]], [0.5], 0, 2) == 0.0
    print("prob:", max_probability(3, [[0, 1], [1, 2], [0, 2]], [0.5, 0.5, 0.3], 0, 2))
