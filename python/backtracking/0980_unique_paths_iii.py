"""
LeetCode 980 - Unique Paths III
Problem: In a grid, 1 = start, 2 = end, 0 = empty, -1 = obstacle. Count
4-directional walks from start to end that visit every non-obstacle cell
exactly once.
Approach: DFS marking cells visited and counting cells still to visit; a walk
counts only if it reaches the end as the last unvisited cell.
Time: O(3^(m*n))  Space: O(m*n)
"""
from typing import List


def unique_paths_iii(grid: List[List[int]]) -> int:
    m, n = len(grid), len(grid[0])
    todo = 0
    sr = sc = 0
    for r in range(m):
        for c in range(n):
            if grid[r][c] != -1:
                todo += 1
            if grid[r][c] == 1:
                sr, sc = r, c

    def dfs(r: int, c: int, left: int) -> int:
        if not (0 <= r < m and 0 <= c < n) or grid[r][c] == -1:
            return 0
        if grid[r][c] == 2:
            return 1 if left == 1 else 0
        saved, grid[r][c] = grid[r][c], -1
        total = sum(dfs(r + dr, c + dc, left - 1)
                    for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)))
        grid[r][c] = saved
        return total

    return dfs(sr, sc, todo)


if __name__ == "__main__":
    assert unique_paths_iii([[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]]) == 2
    assert unique_paths_iii([[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]]) == 4
    assert unique_paths_iii([[0, 1], [2, 0]]) == 0
    print("ok")
