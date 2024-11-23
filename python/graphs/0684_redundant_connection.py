# LeetCode 684 - Redundant Connection
# A tree of n nodes had one extra edge added. Given the edges, return an edge that can be
# removed so the result is a tree; if several, return the one occurring last.
# Approach: union-find; the first edge whose endpoints are already connected is the answer.
# Time: O(n * alpha(n))  Space: O(n)

def find_redundant_connection(edges: list[list[int]]) -> list[int]:
    parent = list(range(len(edges) + 1))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for u, v in edges:
        ru, rv = find(u), find(v)
        if ru == rv:
            return [u, v]
        parent[ru] = rv
    return []


if __name__ == "__main__":
    assert find_redundant_connection([[1, 2], [1, 3], [2, 3]]) == [2, 3]
    assert find_redundant_connection([[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]) == [1, 4]
    print("redundant:", find_redundant_connection([[1, 2], [1, 3], [2, 3]]))
