# LeetCode 547 - Number of Provinces
# isConnected[i][j] = 1 if city i and j are directly connected. A province is a group of
# directly or indirectly connected cities. Return the number of provinces.
# Approach: union-find with path compression and union by size.
# Time: O(n^2 * alpha(n))  Space: O(n)

def find_circle_num(is_connected: list[list[int]]) -> int:
    n = len(is_connected)
    parent, size = list(range(n)), [1] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    comps = n
    for i in range(n):
        for j in range(i + 1, n):
            if is_connected[i][j]:
                a, b = find(i), find(j)
                if a != b:
                    if size[a] < size[b]:
                        a, b = b, a
                    parent[b] = a
                    size[a] += size[b]
                    comps -= 1
    return comps


if __name__ == "__main__":
    assert find_circle_num([[1, 1, 0], [1, 1, 0], [0, 0, 1]]) == 2
    assert find_circle_num([[1, 0, 0], [0, 1, 0], [0, 0, 1]]) == 3
    print("provinces:", find_circle_num([[1, 1, 0], [1, 1, 0], [0, 0, 1]]))
