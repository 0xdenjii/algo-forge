"""
Problem: LeetCode 18
Title: 18. 4Sum

Statement:
    Return all unique quadruplets that sum to target.

Approach:
    Sort; two nested loops with duplicate skipping, inner converging pointers.

Complexity: O(n^3) time, O(1) extra space.
"""

def four_sum(nums: list[int], target: int) -> list[list[int]]:
    nums = sorted(nums)
    n, res = len(nums), []
    for i in range(n - 3):
        if i and nums[i] == nums[i - 1]:
            continue
        for j in range(i + 1, n - 2):
            if j > i + 1 and nums[j] == nums[j - 1]:
                continue
            lo, hi = j + 1, n - 1
            while lo < hi:
                s = nums[i] + nums[j] + nums[lo] + nums[hi]
                if s < target:
                    lo += 1
                elif s > target:
                    hi -= 1
                else:
                    res.append([nums[i], nums[j], nums[lo], nums[hi]])
                    lo += 1
                    hi -= 1
                    while lo < hi and nums[lo] == nums[lo - 1]:
                        lo += 1
    return res


if __name__ == "__main__":
    assert four_sum([1, 0, -1, 0, -2, 2], 0) == [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
    assert four_sum([2, 2, 2, 2, 2], 8) == [[2, 2, 2, 2]]
    print("ok")
