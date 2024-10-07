"""
Problem: LeetCode 16
Title: 16. 3Sum Closest

Statement:
    Find three integers in nums whose sum is closest to target; return that sum.

Approach:
    Sort; for each i converge pointers, keeping the sum with minimal |diff|.

Complexity: O(n^2) time, O(1) space.
"""

def three_sum_closest(nums: list[int], target: int) -> int:
    nums = sorted(nums)
    best = nums[0] + nums[1] + nums[2]
    for i in range(len(nums) - 2):
        lo, hi = i + 1, len(nums) - 1
        while lo < hi:
            s = nums[i] + nums[lo] + nums[hi]
            if abs(s - target) < abs(best - target):
                best = s
            if s == target:
                return s
            if s < target:
                lo += 1
            else:
                hi -= 1
    return best


if __name__ == "__main__":
    assert three_sum_closest([-1, 2, 1, -4], 1) == 2
    assert three_sum_closest([0, 0, 0], 1) == 0
    print("ok")
