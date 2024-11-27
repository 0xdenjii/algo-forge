# Bellman-Ford Single-Source Shortest Paths (classic algorithm)
# Given a directed graph with possibly negative weights and a source, compute shortest
# distances, or report that a negative cycle is reachable from the source.
# Approach: relax all edges V-1 times; a further successful relaxation means a negative cycle.
# Time: O(V * E)  Space: O(V)

INF = float("inf")


def bellman_ford(n: int, edges: list[tuple[int, int, float]], src: int):
    dist = [INF] * n
    dist[src] = 0
    for _ in range(n - 1):
        changed = False
        for u, v, w in edges:
            if dist[u] != INF and dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                changed = True
        if not changed:
            break
    for u, v, w in edges:
        if dist[u] != INF and dist[u] + w < dist[v]:
            return None
    return dist


if __name__ == "__main__":
    e = [(0, 1, 4), (0, 2, 5), (1, 2, -3), (2, 3, 4), (3, 1, 2)]
    assert bellman_ford(4, e, 0) == [0, 4, 1, 5]
    assert bellman_ford(3, [(0, 1, 1), (1, 2, -2), (2, 1, 1)], 0) is None
    print("dist:", bellman_ford(4, e, 0))
