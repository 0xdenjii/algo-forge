# LeetCode 518 - Coin Change II
# Problem: Count the number of combinations of coins (unlimited supply) that
#   make up the given amount. Order does not matter.
# Approach: Iterate coins in the outer loop so each combination is counted once:
#   dp[a] += dp[a - c].
# Time: O(amount * len(coins))  Space: O(amount)
from typing import Sequence


def change(amount: int, coins: Sequence[int]) -> int:
    dp = [1] + [0] * amount
    for c in coins:
        for a in range(c, amount + 1):
            dp[a] += dp[a - c]
    return dp[amount]


if __name__ == "__main__":
    assert change(5, [1, 2, 5]) == 4
    assert change(3, [2]) == 0
    assert change(10, [10]) == 1
    print("ok")
