# LeetCode 322 - Coin Change
# Problem: Given coin denominations and an amount, return the fewest coins that
#   sum to amount, or -1 if impossible (unlimited supply of each coin).
# Approach: Unbounded knapsack: dp[a] = min(dp[a - c] + 1) over coins c.
# Time: O(amount * len(coins))  Space: O(amount)
from typing import Sequence


def coin_change(coins: Sequence[int], amount: int) -> int:
    inf = amount + 1
    dp = [0] + [inf] * amount
    for a in range(1, amount + 1):
        for c in coins:
            if c <= a and dp[a - c] + 1 < dp[a]:
                dp[a] = dp[a - c] + 1
    return dp[amount] if dp[amount] != inf else -1


if __name__ == "__main__":
    assert coin_change([1, 2, 5], 11) == 3
    assert coin_change([2], 3) == -1
    assert coin_change([1], 0) == 0
    print("ok")
