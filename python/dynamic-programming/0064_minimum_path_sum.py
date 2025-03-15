# LeetCode 64 - Minimum Path Sum
# Problem: Given a grid of non-negative numbers, find a path from top-left to
#   bottom-right (moving right or down) minimising the sum along the path.
# Approach: dp[j] = grid[i][j] + min(dp[j] (from above), dp[j-1] (from left)).
# Time: O(m * n)  Space: O(n)
from math import inf


def min_path_sum(grid: list[list[int]]) -> int:
    n = len(grid[0])
    dp: list[float] = [inf] * n
    dp[0] = 0
    for row in grid:
        dp[0] += row[0]
        for j in range(1, n):
            dp[j] = row[j] + min(dp[j], dp[j - 1])
    return int(dp[-1])


if __name__ == "__main__":
    assert min_path_sum([[1, 3, 1], [1, 5, 1], [4, 2, 1]]) == 7
    assert min_path_sum([[1, 2, 3], [4, 5, 6]]) == 12
    print("ok")
