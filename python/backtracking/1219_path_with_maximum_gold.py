"""
LeetCode 1219 - Path with Maximum Gold
Problem: In a grid of gold amounts (0 = empty), start at any gold cell and
walk 4-directionally, never revisiting a cell or stepping on 0. Return the
maximum gold collectable.
Approach: DFS from each gold cell, temporarily zeroing visited cells.
Time: O(k * 3^k) for k gold cells  Space: O(k)
"""
from typing import List


def get_maximum_gold(grid: List[List[int]]) -> int:
    m, n = len(grid), len(grid[0])

    def dfs(r: int, c: int) -> int:
        if not (0 <= r < m and 0 <= c < n) or grid[r][c] == 0:
            return 0
        g, grid[r][c] = grid[r][c], 0
        best = max(dfs(r + 1, c), dfs(r - 1, c), dfs(r, c + 1), dfs(r, c - 1))
        grid[r][c] = g
        return g + best

    return max((dfs(r, c) for r in range(m) for c in range(n)), default=0)


if __name__ == "__main__":
    assert get_maximum_gold([[0, 6, 0], [5, 8, 7], [0, 9, 0]]) == 24
    assert get_maximum_gold([[1, 0, 7], [2, 0, 6], [3, 4, 5], [0, 3, 0], [9, 0, 20]]) == 28
    print("ok")
