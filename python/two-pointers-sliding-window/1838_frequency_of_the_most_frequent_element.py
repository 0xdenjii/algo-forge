"""
Problem: LeetCode 1838
Title: 1838. Frequency of the Most Frequent Element

Statement:
    With at most k increments (+1 to any element), maximise the frequency of
    some value.

Approach:
    Sort; window [l, r] can all become nums[r] iff nums[r]*len - sum <= k.

Complexity: O(n log n) time, O(1) extra space.
"""

def max_frequency(nums: list[int], k: int) -> int:
    nums = sorted(nums)
    left = total = best = 0
    for right, x in enumerate(nums):
        total += x
        while x * (right - left + 1) - total > k:
            total -= nums[left]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    assert max_frequency([1, 2, 4], 5) == 3
    assert max_frequency([1, 4, 8, 13], 5) == 2
    assert max_frequency([3, 9, 6], 2) == 1
    print("ok")
