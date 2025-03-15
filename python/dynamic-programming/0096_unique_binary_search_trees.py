# LeetCode 96 - Unique Binary Search Trees
# Problem: Count structurally unique BSTs that store values 1..n.
# Approach: Catalan recurrence: G(n) = sum over root r of G(r-1) * G(n-r).
# Time: O(n^2)  Space: O(n)


def num_trees(n: int) -> int:
    g = [1] + [0] * n
    for nodes in range(1, n + 1):
        g[nodes] = sum(g[r - 1] * g[nodes - r] for r in range(1, nodes + 1))
    return g[n]


if __name__ == "__main__":
    assert num_trees(3) == 5
    assert num_trees(1) == 1
    assert num_trees(19) == 1767263190
    print("ok")
