"""LeetCode 373 - Find K Pairs with Smallest Sums

Given two ascending arrays nums1 and nums2, return the k pairs (u, v) with
u from nums1 and v from nums2 that have the smallest sums.

Approach: seed the heap with (nums1[i] + nums2[0]) for each i; popping (i, j)
pushes (i, j + 1). Every pair enters the heap at most once.
Time: O(k log k)  Space: O(k)
"""
import heapq


def k_smallest_pairs(nums1: list[int], nums2: list[int], k: int) -> list[list[int]]:
    if not nums1 or not nums2:
        return []
    heap = [(nums1[i] + nums2[0], i, 0) for i in range(min(k, len(nums1)))]
    heapq.heapify(heap)
    out: list[list[int]] = []
    while heap and len(out) < k:
        _, i, j = heapq.heappop(heap)
        out.append([nums1[i], nums2[j]])
        if j + 1 < len(nums2):
            heapq.heappush(heap, (nums1[i] + nums2[j + 1], i, j + 1))
    return out


if __name__ == "__main__":
    assert k_smallest_pairs([1, 7, 11], [2, 4, 6], 3) == [[1, 2], [1, 4], [1, 6]]
    assert k_smallest_pairs([1, 1, 2], [1, 2, 3], 2) == [[1, 1], [1, 1]]
    print("ok")
