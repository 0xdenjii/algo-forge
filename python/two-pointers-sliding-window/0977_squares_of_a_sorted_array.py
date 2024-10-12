"""
Problem: LeetCode 977
Title: 977. Squares of a Sorted Array

Statement:
    Given a sorted array (possibly with negatives), return the sorted squares.

Approach:
    Largest square sits at one of the ends; fill output from the back.

Complexity: O(n) time, O(n) output space.
"""

def sorted_squares(nums: list[int]) -> list[int]:
    n = len(nums)
    res = [0] * n
    lo, hi = 0, n - 1
    for k in range(n - 1, -1, -1):
        if abs(nums[lo]) > abs(nums[hi]):
            res[k] = nums[lo] ** 2
            lo += 1
        else:
            res[k] = nums[hi] ** 2
            hi -= 1
    return res


if __name__ == "__main__":
    assert sorted_squares([-4, -1, 0, 3, 10]) == [0, 1, 9, 16, 100]
    assert sorted_squares([-7, -3, 2, 3, 11]) == [4, 9, 9, 49, 121]
    print("ok")
