"""
LeetCode 77 - Combinations
Problem: Given n and k, return all combinations of k numbers chosen from 1..n.
Approach: DFS with start index; prune when not enough numbers remain
(i may go only up to n - need + 1).
Time: O(k * C(n, k))  Space: O(k)
"""
from typing import List


def combine(n: int, k: int) -> List[List[int]]:
    out: List[List[int]] = []
    path: List[int] = []

    def dfs(start: int) -> None:
        need = k - len(path)
        if need == 0:
            out.append(path.copy())
            return
        for i in range(start, n - need + 2):
            path.append(i)
            dfs(i + 1)
            path.pop()

    dfs(1)
    return out


if __name__ == "__main__":
    assert combine(4, 2) == [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    assert combine(1, 1) == [[1]]
    assert len(combine(10, 3)) == 120
    print("ok")
