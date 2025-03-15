# LeetCode 123 - Best Time to Buy and Sell Stock III
# Problem: At most two transactions. Maximise profit.
# Approach: Four running states: after first buy, first sell, second buy,
#   second sell; each updated greedily per price.
# Time: O(n)  Space: O(1)
from math import inf


def max_profit(prices: list[int]) -> int:
    b1 = b2 = -inf
    s1 = s2 = 0
    for p in prices:
        b1 = max(b1, -p)
        s1 = max(s1, b1 + p)
        b2 = max(b2, s1 - p)
        s2 = max(s2, b2 + p)
    return int(s2)


if __name__ == "__main__":
    assert max_profit([3, 3, 5, 0, 0, 3, 1, 4]) == 6
    assert max_profit([1, 2, 3, 4, 5]) == 4
    assert max_profit([7, 6, 4, 3, 1]) == 0
    print("ok")
