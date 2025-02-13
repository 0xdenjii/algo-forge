"""LeetCode 632 - Smallest Range Covering Elements from K Lists

Given k sorted lists, find the smallest range [a, b] that includes at least
one number from each list (ties broken by smaller a).

Approach: the heap holds one pointer per list and we track the current max.
[heap min, max] always covers every list; advance the list owning the min.
Time: O(N log k)  Space: O(k)
"""
import heapq


def smallest_range(nums: list[list[int]]) -> list[int]:
    heap = [(lst[0], i, 0) for i, lst in enumerate(nums)]
    heapq.heapify(heap)
    hi = max(lst[0] for lst in nums)
    best = [heap[0][0], hi]
    while True:
        lo, i, j = heapq.heappop(heap)
        if hi - lo < best[1] - best[0]:
            best = [lo, hi]
        if j + 1 == len(nums[i]):
            return best
        nxt = nums[i][j + 1]
        hi = max(hi, nxt)
        heapq.heappush(heap, (nxt, i, j + 1))


if __name__ == "__main__":
    assert smallest_range([[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]]) == [20, 24]
    assert smallest_range([[1, 2, 3], [1, 2, 3], [1, 2, 3]]) == [1, 1]
    print("ok")
