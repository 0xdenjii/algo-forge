# Problem: LeetCode 454 - 4Sum II
#
# Given four equal-length arrays, count tuples (i, j, k, l) with
# a[i] + b[j] + c[k] + d[l] == 0.
#
# Approach: Meet in the middle: count all a+b sums, then look up -(c+d).
# Complexity: O(n^2) time, O(n^2) space

from collections import Counter


def four_sum_count(a: list[int], b: list[int], c: list[int], d: list[int]) -> int:
    ab = Counter(x + y for x in a for y in b)
    return sum(ab[-(x + y)] for x in c for y in d)


if __name__ == "__main__":
    assert four_sum_count([1, 2], [-2, -1], [-1, 2], [0, 2]) == 2
    assert four_sum_count([0], [0], [0], [0]) == 1
    print("ok")
