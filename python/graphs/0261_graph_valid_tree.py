# LeetCode 261 - Graph Valid Tree
# Given n nodes and a list of undirected edges, decide whether they form a valid tree:
# connected and acyclic.
# Approach: a tree has exactly n-1 edges; then union-find must never see a cycle.
# Time: O(n * alpha(n))  Space: O(n)

def valid_tree(n: int, edges: list[list[int]]) -> bool:
    if len(edges) != n - 1:
        return False
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return False
        parent[ra] = rb
    return True


if __name__ == "__main__":
    assert valid_tree(5, [[0, 1], [0, 2], [0, 3], [1, 4]])
    assert not valid_tree(5, [[0, 1], [1, 2], [2, 3], [1, 3], [1, 4]])
    print("valid tree:", valid_tree(5, [[0, 1], [0, 2], [0, 3], [1, 4]]))
