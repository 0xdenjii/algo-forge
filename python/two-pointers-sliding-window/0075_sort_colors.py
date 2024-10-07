"""
Problem: LeetCode 75
Title: 75. Sort Colors

Statement:
    Sort an array of 0s, 1s and 2s in place in one pass.

Approach:
    Maintain regions [0,lo)=0, [lo,mid)=1, (hi,end]=2; classify nums[mid].

Complexity: O(n) time, O(1) space.
"""

def sort_colors(nums: list[int]) -> None:
    lo = mid = 0
    hi = len(nums) - 1
    while mid <= hi:
        if nums[mid] == 0:
            nums[lo], nums[mid] = nums[mid], nums[lo]
            lo += 1
            mid += 1
        elif nums[mid] == 1:
            mid += 1
        else:
            nums[mid], nums[hi] = nums[hi], nums[mid]
            hi -= 1


if __name__ == "__main__":
    a = [2, 0, 2, 1, 1, 0]
    sort_colors(a)
    assert a == [0, 0, 1, 1, 2, 2]
    print("ok")
