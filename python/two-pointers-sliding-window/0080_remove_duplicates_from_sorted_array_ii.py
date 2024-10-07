"""
Problem: LeetCode 80
Title: 80. Remove Duplicates from Sorted Array II

Statement:
    Remove duplicates in place so each element appears at most twice; return k.

Approach:
    Write x if w < 2 or x != nums[w-2].

Complexity: O(n) time, O(1) space.
"""

def remove_duplicates(nums: list[int]) -> int:
    w = 0
    for x in nums:
        if w < 2 or x != nums[w - 2]:
            nums[w] = x
            w += 1
    return w


if __name__ == "__main__":
    a = [0, 0, 1, 1, 1, 1, 2, 3, 3]
    k = remove_duplicates(a)
    assert a[:k] == [0, 0, 1, 1, 2, 3, 3]
    print("ok")
