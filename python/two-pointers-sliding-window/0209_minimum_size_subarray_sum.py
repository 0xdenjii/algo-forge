"""
Problem: LeetCode 209
Title: 209. Minimum Size Subarray Sum

Statement:
    Given positive integers and a target, return the minimal length of a
    contiguous subarray whose sum is >= target, or 0.

Approach:
    Grow right; while sum >= target, record length and shrink left.

Complexity: O(n) time, O(1) space.
"""

def min_sub_array_len(target: int, nums: list[int]) -> int:
    left = total = 0
    best = len(nums) + 1
    for right, x in enumerate(nums):
        total += x
        while total >= target:
            best = min(best, right - left + 1)
            total -= nums[left]
            left += 1
    return 0 if best > len(nums) else best


if __name__ == "__main__":
    assert min_sub_array_len(7, [2, 3, 1, 2, 4, 3]) == 2
    assert min_sub_array_len(4, [1, 4, 4]) == 1
    assert min_sub_array_len(11, [1, 1, 1, 1, 1, 1, 1, 1]) == 0
    print("ok")
