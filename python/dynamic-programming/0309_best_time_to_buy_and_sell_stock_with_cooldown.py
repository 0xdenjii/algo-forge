# LeetCode 309 - Best Time to Buy and Sell Stock with Cooldown
# Problem: Unlimited transactions, but after selling you must wait one day before
#   buying again. Maximise profit.
# Approach: State machine with three states per day: hold, sold (just sold),
#   rest (free to buy).
# Time: O(n)  Space: O(1)
from math import inf


def max_profit(prices: list[int]) -> int:
    hold, sold, rest = -inf, 0, 0
    for p in prices:
        hold, sold, rest = max(hold, rest - p), hold + p, max(rest, sold)
    return int(max(sold, rest))


if __name__ == "__main__":
    assert max_profit([1, 2, 3, 0, 2]) == 3
    assert max_profit([1]) == 0
    print("ok")
