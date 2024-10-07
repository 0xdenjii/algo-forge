"""
Problem: LeetCode 26
Title: 26. Remove Duplicates from Sorted Array

Statement:
    Remove duplicates in place from a sorted array so each element appears once;
    return the new length k (first k slots hold the result).

Approach:
    Write pointer advances only when a new value is seen.

Complexity: O(n) time, O(1) space.
"""

def remove_duplicates(nums: list[int]) -> int:
    if not nums:
        return 0
    w = 1
    for r in range(1, len(nums)):
        if nums[r] != nums[w - 1]:
            nums[w] = nums[r]
            w += 1
    return w


if __name__ == "__main__":
    a = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
    k = remove_duplicates(a)
    assert k == 5 and a[:k] == [0, 1, 2, 3, 4]
    assert remove_duplicates([]) == 0
    print("ok")
