"""LeetCode 215 - Kth Largest Element in an Array

Given an integer array nums and an integer k, return the kth largest element
in the array (in sorted order, not the kth distinct element).

Approach: keep a min-heap of size k; its root is the kth largest.
Time: O(n log k)  Space: O(k)
"""
import heapq


def find_kth_largest(nums: list[int], k: int) -> int:
    heap: list[int] = []
    for x in nums:
        if len(heap) < k:
            heapq.heappush(heap, x)
        elif x > heap[0]:
            heapq.heapreplace(heap, x)
    return heap[0]


if __name__ == "__main__":
    assert find_kth_largest([3, 2, 1, 5, 6, 4], 2) == 5
    assert find_kth_largest([3, 2, 3, 1, 2, 4, 5, 5, 6], 4) == 4
    print("ok")
