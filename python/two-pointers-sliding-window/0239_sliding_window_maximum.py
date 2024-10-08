"""
Problem: LeetCode 239
Title: 239. Sliding Window Maximum

Statement:
    Return the maximum of every contiguous window of size k.

Approach:
    Deque holds indices with decreasing values; front is the window max, popped
    when it falls out.

Complexity: O(n) time, O(k) space.
"""

from collections import deque


def max_sliding_window(nums: list[int], k: int) -> list[int]:
    dq: deque[int] = deque()
    res = []
    for i, x in enumerate(nums):
        while dq and nums[dq[-1]] <= x:
            dq.pop()
        dq.append(i)
        if dq[0] <= i - k:
            dq.popleft()
        if i >= k - 1:
            res.append(nums[dq[0]])
    return res


if __name__ == "__main__":
    assert max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3) == [3, 3, 5, 5, 6, 7]
    assert max_sliding_window([1], 1) == [1]
    print("ok")
