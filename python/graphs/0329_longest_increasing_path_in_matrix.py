# LeetCode 329 - Longest Increasing Path in a Matrix
# Return the length of the longest strictly increasing path in an integer matrix,
# moving in 4 directions.
# Approach: cells form a DAG (edges to larger neighbors); memoized DFS gives longest path.
# Time: O(m*n)  Space: O(m*n)

from functools import lru_cache
import sys


def longest_increasing_path(matrix: list[list[int]]) -> int:
    m, n = len(matrix), len(matrix[0])
    sys.setrecursionlimit(max(10000, m * n + 100))

    @lru_cache(maxsize=None)
    def dfs(r, c):
        best = 1
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < m and 0 <= nc < n and matrix[nr][nc] > matrix[r][c]:
                best = max(best, 1 + dfs(nr, nc))
        return best

    return max(dfs(i, j) for i in range(m) for j in range(n))


if __name__ == "__main__":
    assert longest_increasing_path([[9, 9, 4], [6, 6, 8], [2, 1, 1]]) == 4
    assert longest_increasing_path([[3, 4, 5], [3, 2, 6], [2, 2, 1]]) == 4
    print("longest:", longest_increasing_path([[9, 9, 4], [6, 6, 8], [2, 1, 1]]))
