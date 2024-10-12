"""
Problem: LeetCode 1004
Title: 1004. Max Consecutive Ones III

Statement:
    Return the max number of consecutive 1s if you may flip at most k zeros.

Approach:
    Count zeros in the window; shrink while zeros > k.

Complexity: O(n) time, O(1) space.
"""

def longest_ones(nums: list[int], k: int) -> int:
    left = zeros = best = 0
    for right, x in enumerate(nums):
        zeros += x == 0
        while zeros > k:
            zeros -= nums[left] == 0
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    assert longest_ones([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2) == 6
    assert longest_ones([0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3) == 10
    print("ok")
