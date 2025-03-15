# LeetCode 62 - Unique Paths
# Problem: A robot at the top-left of an m x n grid moves only right or down.
#   Count paths to the bottom-right corner.
# Approach: dp[j] += dp[j-1] row by row (paths from above + from left).
# Time: O(m * n)  Space: O(n)


def unique_paths(m: int, n: int) -> int:
    dp = [1] * n
    for _ in range(1, m):
        for j in range(1, n):
            dp[j] += dp[j - 1]
    return dp[-1]


if __name__ == "__main__":
    assert unique_paths(3, 7) == 28
    assert unique_paths(3, 2) == 3
    assert unique_paths(1, 1) == 1
    print("ok")
