"""
Problem: LeetCode 15
Title: 15. 3Sum

Statement:
    Return all unique triplets [a, b, c] in nums such that a + b + c == 0.

Approach:
    Sort; fix each i, run converging pointers on the suffix, skipping duplicates.

Complexity: O(n^2) time, O(1) extra space (excluding output/sort).
"""

def three_sum(nums: list[int]) -> list[list[int]]:
    nums = sorted(nums)
    n, res = len(nums), []
    for i in range(n - 2):
        if nums[i] > 0:
            break
        if i and nums[i] == nums[i - 1]:
            continue
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = nums[i] + nums[lo] + nums[hi]
            if s < 0:
                lo += 1
            elif s > 0:
                hi -= 1
            else:
                res.append([nums[i], nums[lo], nums[hi]])
                lo += 1
                hi -= 1
                while lo < hi and nums[lo] == nums[lo - 1]:
                    lo += 1
    return res


if __name__ == "__main__":
    assert three_sum([-1, 0, 1, 2, -1, -4]) == [[-1, -1, 2], [-1, 0, 1]]
    assert three_sum([0, 0, 0, 0]) == [[0, 0, 0]]
    assert three_sum([0, 1, 1]) == []
    print("ok")
