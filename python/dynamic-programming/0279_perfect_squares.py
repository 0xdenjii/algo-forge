# LeetCode 279 - Perfect Squares
# Problem: Return the least number of perfect squares that sum to n.
# Approach: dp[i] = 1 + min(dp[i - s]) over squares s <= i (coin change with
#   square coins).
# Time: O(n * sqrt(n))  Space: O(n)
from math import isqrt


def num_squares(n: int) -> int:
    squares = [k * k for k in range(1, isqrt(n) + 1)]
    dp = [0] + [n] * n
    for i in range(1, n + 1):
        for s in squares:
            if s > i:
                break
            dp[i] = min(dp[i], dp[i - s] + 1)
    return dp[n]


if __name__ == "__main__":
    assert num_squares(12) == 3
    assert num_squares(13) == 2
    assert num_squares(1) == 1
    print("ok")
