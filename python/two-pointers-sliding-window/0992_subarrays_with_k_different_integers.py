"""
Problem: LeetCode 992
Title: 992. Subarrays with K Different Integers

Statement:
    Count subarrays containing exactly k distinct integers.

Approach:
    atMost(k) counts windows with <= k distinct via the standard window; take
    the difference.

Complexity: O(n) time, O(k) space.
"""

from collections import defaultdict


def _at_most(nums: list[int], k: int) -> int:
    count: defaultdict[int, int] = defaultdict(int)
    left = total = 0
    for right, x in enumerate(nums):
        count[x] += 1
        while len(count) > k:
            count[nums[left]] -= 1
            if count[nums[left]] == 0:
                del count[nums[left]]
            left += 1
        total += right - left + 1
    return total


def subarrays_with_k_distinct(nums: list[int], k: int) -> int:
    return _at_most(nums, k) - _at_most(nums, k - 1)


if __name__ == "__main__":
    assert subarrays_with_k_distinct([1, 2, 1, 2, 3], 2) == 7
    assert subarrays_with_k_distinct([1, 2, 1, 3, 4], 3) == 3
    print("ok")
