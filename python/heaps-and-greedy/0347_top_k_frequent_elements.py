"""LeetCode 347 - Top K Frequent Elements

Given an integer array nums and an integer k, return the k most frequent
elements, in any order.

Approach: count with Counter, then heapq.nlargest over the counts.
Time: O(n log k)  Space: O(n)
"""
import heapq
from collections import Counter


def top_k_frequent(nums: list[int], k: int) -> list[int]:
    counts = Counter(nums)
    return heapq.nlargest(k, counts, key=counts.__getitem__)


if __name__ == "__main__":
    assert sorted(top_k_frequent([1, 1, 1, 2, 2, 3], 2)) == [1, 2]
    assert top_k_frequent([1], 1) == [1]
    print("ok")
