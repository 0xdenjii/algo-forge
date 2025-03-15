# LeetCode 188 - Best Time to Buy and Sell Stock IV
# Problem: At most k transactions (buy then sell, no overlap). Maximise profit.
# Approach: buy[t] / sell[t] = best balance after the t-th buy / sell. If
#   k >= n/2 the limit is irrelevant, so sum every positive day-to-day gain.
# Time: O(n * k)  Space: O(k)
from math import inf


def max_profit(k: int, prices: list[int]) -> int:
    if k >= len(prices) // 2:
        return sum(max(0, b - a) for a, b in zip(prices, prices[1:]))
    buy = [-inf] * (k + 1)
    sell = [0] * (k + 1)
    for p in prices:
        for t in range(1, k + 1):
            buy[t] = max(buy[t], sell[t - 1] - p)
            sell[t] = max(sell[t], buy[t] + p)
    return int(sell[k])


if __name__ == "__main__":
    assert max_profit(2, [2, 4, 1]) == 2
    assert max_profit(2, [3, 2, 6, 5, 0, 3]) == 7
    assert max_profit(0, [1, 3]) == 0
    print("ok")
