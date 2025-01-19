"""LeetCode 122 - Best Time to Buy and Sell Stock II

Given daily prices, you may hold at most one share and trade any number of
times. Return the maximum profit.

Approach: greedy - collect every positive day-to-day increase.
Time: O(n)  Space: O(1)
"""
from itertools import pairwise


def max_profit(prices: list[int]) -> int:
    return sum(max(0, b - a) for a, b in pairwise(prices))


if __name__ == "__main__":
    assert max_profit([7, 1, 5, 3, 6, 4]) == 7
    assert max_profit([1, 2, 3, 4, 5]) == 4
    assert max_profit([7, 6, 4, 3, 1]) == 0
    print("ok")
