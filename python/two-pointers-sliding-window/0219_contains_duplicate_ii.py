"""
Problem: LeetCode 219
Title: 219. Contains Duplicate II

Statement:
    Return True if there are i != j with nums[i] == nums[j] and |i - j| <= k.

Approach:
    Keep a set of the last k values; evict the one leaving the window.

Complexity: O(n) time, O(k) space.
"""

def contains_nearby_duplicate(nums: list[int], k: int) -> bool:
    window: set[int] = set()
    for i, x in enumerate(nums):
        if x in window:
            return True
        window.add(x)
        if len(window) > k:
            window.remove(nums[i - k])
    return False


if __name__ == "__main__":
    assert contains_nearby_duplicate([1, 2, 3, 1], 3)
    assert contains_nearby_duplicate([1, 0, 1, 1], 1)
    assert not contains_nearby_duplicate([1, 2, 3, 1, 2, 3], 2)
    print("ok")
