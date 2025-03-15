# LeetCode 120 - Triangle
# Problem: Given a triangle array, return the minimum path sum from top to bottom,
#   moving to an adjacent index on the row below at each step.
# Approach: Bottom-up: dp[j] = row[j] + min(dp[j], dp[j+1]) starting from the last row.
# Time: O(n^2)  Space: O(n)


def minimum_total(triangle: list[list[int]]) -> int:
    dp = list(triangle[-1])
    for row in reversed(triangle[:-1]):
        for j, v in enumerate(row):
            dp[j] = v + min(dp[j], dp[j + 1])
    return dp[0]


if __name__ == "__main__":
    assert minimum_total([[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]) == 11
    assert minimum_total([[-10]]) == -10
    print("ok")
