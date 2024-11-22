# LeetCode 310 - Minimum Height Trees
# Given a tree of n nodes, return all roots that produce a tree of minimum height.
# Approach: repeatedly strip leaves (topological trimming); the last 1-2 nodes are centroids.
# Time: O(n)  Space: O(n)

def find_min_height_trees(n: int, edges: list[list[int]]) -> list[int]:
    if n <= 2:
        return list(range(n))
    adj = [set() for _ in range(n)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    leaves = [i for i in range(n) if len(adj[i]) == 1]
    remaining = n
    while remaining > 2:
        remaining -= len(leaves)
        nxt = []
        for leaf in leaves:
            nb = adj[leaf].pop()
            adj[nb].discard(leaf)
            if len(adj[nb]) == 1:
                nxt.append(nb)
        leaves = nxt
    return sorted(leaves)


if __name__ == "__main__":
    assert find_min_height_trees(4, [[1, 0], [1, 2], [1, 3]]) == [1]
    assert find_min_height_trees(6, [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]) == [3, 4]
    print("roots:", find_min_height_trees(6, [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]))
