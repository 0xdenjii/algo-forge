# Kruskal's Minimum Spanning Tree (classic algorithm)
# Given an undirected weighted graph, find a spanning forest of minimum total weight.
# Approach: sort edges by weight; add each edge whose endpoints are in different components.
# Time: O(E log E)  Space: O(V)

def kruskal(n: int, edges: list[tuple[int, int, int]]):
    parent, rank = list(range(n)), [0] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    total, chosen = 0, []
    for u, v, w in sorted(edges, key=lambda e: e[2]):
        ru, rv = find(u), find(v)
        if ru == rv:
            continue
        if rank[ru] < rank[rv]:
            ru, rv = rv, ru
        parent[rv] = ru
        if rank[ru] == rank[rv]:
            rank[ru] += 1
        total += w
        chosen.append((u, v, w))
    return total, chosen


if __name__ == "__main__":
    edges = [(0, 1, 10), (0, 2, 6), (0, 3, 5), (1, 3, 15), (2, 3, 4)]
    total, chosen = kruskal(4, edges)
    assert total == 19 and len(chosen) == 3
    print("mst:", total, chosen)
