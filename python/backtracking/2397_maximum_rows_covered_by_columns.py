"""
LeetCode 2397 - Maximum Rows Covered by Columns
Problem: Given a binary matrix and numSelect, choose exactly numSelect
columns. A row is covered if all its 1s lie in chosen columns. Maximize the
number of covered rows.
Approach: Encode rows as bitmasks; backtrack over column choices (C(n, k)
leaves), counting rows whose mask is a subset of the chosen mask.
Time: O(C(n, k) * m)  Space: O(n)
"""
from typing import List


def maximum_rows(matrix: List[List[int]], num_select: int) -> int:
    n = len(matrix[0])
    rows = [sum(bit << j for j, bit in enumerate(r)) for r in matrix]
    best = 0

    def dfs(col: int, left: int, chosen: int) -> None:
        nonlocal best
        if left == 0:
            best = max(best, sum(1 for r in rows if r & ~chosen == 0))
            return
        if n - col < left:
            return
        dfs(col + 1, left - 1, chosen | (1 << col))
        dfs(col + 1, left, chosen)

    dfs(0, num_select, 0)
    return best


if __name__ == "__main__":
    assert maximum_rows([[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 0, 1]], 2) == 3
    assert maximum_rows([[1], [0]], 1) == 2
    print("ok")
