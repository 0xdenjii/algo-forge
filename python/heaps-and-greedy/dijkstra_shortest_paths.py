"""Dijkstra's Shortest Paths (greedy with a binary heap)

Given a directed graph with non-negative edge weights, compute the shortest
distance from a source to every vertex.

Approach: greedily settle the closest unsettled vertex using a min-heap;
stale heap entries are skipped (lazy deletion).
Time: O((V + E) log V)  Space: O(V + E)
"""
import heapq
from collections import defaultdict


def dijkstra(n: int, edges: list[tuple[int, int, int]], src: int) -> list[float]:
    graph: defaultdict[int, list[tuple[int, int]]] = defaultdict(list)
    for u, v, w in edges:
        graph[u].append((v, w))
    dist = [float("inf")] * n
    dist[src] = 0
    heap = [(0, src)]
    while heap:
        d, u = heapq.heappop(heap)
        if d > dist[u]:
            continue
        for v, w in graph[u]:
            nd = d + w
            if nd < dist[v]:
                dist[v] = nd
                heapq.heappush(heap, (nd, v))
    return dist


if __name__ == "__main__":
    edges = [(0, 1, 4), (0, 2, 1), (2, 1, 2), (1, 3, 1), (2, 3, 5)]
    assert dijkstra(5, edges, 0) == [0, 3, 1, 4, float("inf")]
    print("ok")
