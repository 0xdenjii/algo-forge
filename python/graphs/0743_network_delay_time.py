# LeetCode 743 - Network Delay Time
# Given directed weighted edges times[i] = (u, v, w) on nodes 1..n and a source k,
# return the time for all nodes to receive the signal, or -1 if some cannot.
# Approach: Dijkstra with a binary heap; answer is the max shortest distance.
# Time: O(E log V)  Space: O(V + E)

import heapq
from collections import defaultdict


def network_delay_time(times: list[list[int]], n: int, k: int) -> int:
    adj = defaultdict(list)
    for u, v, w in times:
        adj[u].append((v, w))
    dist = {}
    pq = [(0, k)]
    while pq:
        d, u = heapq.heappop(pq)
        if u in dist:
            continue
        dist[u] = d
        for v, w in adj[u]:
            if v not in dist:
                heapq.heappush(pq, (d + w, v))
    return max(dist.values()) if len(dist) == n else -1


if __name__ == "__main__":
    assert network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2) == 2
    assert network_delay_time([[1, 2, 1]], 2, 2) == -1
    print("delay:", network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2))
