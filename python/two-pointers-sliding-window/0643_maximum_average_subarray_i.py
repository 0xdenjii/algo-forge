"""
Problem: LeetCode 643
Title: 643. Maximum Average Subarray I

Statement:
    Find the contiguous subarray of length k with the maximum average.

Approach:
    Maintain a running sum; add the incoming and subtract the outgoing element.

Complexity: O(n) time, O(1) space.
"""

def find_max_average(nums: list[int], k: int) -> float:
    cur = best = sum(nums[:k])
    for i in range(k, len(nums)):
        cur += nums[i] - nums[i - k]
        best = max(best, cur)
    return best / k


if __name__ == "__main__":
    assert find_max_average([1, 12, -5, -6, 50, 3], 4) == 12.75
    assert find_max_average([5], 1) == 5.0
    print("ok")
