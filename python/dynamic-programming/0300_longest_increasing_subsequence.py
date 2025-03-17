# LeetCode 300 - Longest Increasing Subsequence
# Problem: Return the length of the longest strictly increasing subsequence.
# Approach: Patience sorting: tails[k] = smallest tail of an increasing
#   subsequence of length k+1; binary search where each element belongs.
# Time: O(n log n)  Space: O(n)
from bisect import bisect_left
from typing import Sequence


def length_of_lis(nums: Sequence[int]) -> int:
    tails: list[int] = []
    for x in nums:
        i = bisect_left(tails, x)
        if i == len(tails):
            tails.append(x)
        else:
            tails[i] = x
    return len(tails)


if __name__ == "__main__":
    assert length_of_lis([10, 9, 2, 5, 3, 7, 101, 18]) == 4
    assert length_of_lis([0, 1, 0, 3, 2, 3]) == 4
    assert length_of_lis([7, 7, 7]) == 1
    print("ok")
