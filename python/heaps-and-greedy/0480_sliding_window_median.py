"""LeetCode 480 - Sliding Window Median

Return the median of every window of size k sliding across nums.

Approach: two heaps with lazy deletion. Outgoing elements are recorded in a
counter and discarded only when they surface at a heap top; a per-step
balance value tracks the logical size difference between the halves.
Time: O(n log n)  Space: O(n)
"""
import heapq
from collections import defaultdict


def median_sliding_window(nums: list[int], k: int) -> list[float]:
    low: list[int] = []   # max-heap (negated), holds ceil(k/2) valid items
    high: list[int] = []  # min-heap
    for x in nums[:k]:
        heapq.heappush(low, -x)
    for _ in range(k // 2):
        heapq.heappush(high, -heapq.heappop(low))

    delayed: defaultdict[int, int] = defaultdict(int)

    def prune(heap: list[int], sign: int) -> None:
        while heap and delayed[sign * heap[0]]:
            delayed[sign * heap[0]] -= 1
            heapq.heappop(heap)

    def median() -> float:
        return float(-low[0]) if k % 2 else (-low[0] + high[0]) / 2

    result = [median()]
    for i in range(k, len(nums)):
        out, inc = nums[i - k], nums[i]
        delayed[out] += 1
        balance = -1 if out <= -low[0] else 1
        if inc <= -low[0]:
            heapq.heappush(low, -inc)
            balance += 1
        else:
            heapq.heappush(high, inc)
            balance -= 1
        if balance > 0:
            heapq.heappush(high, -heapq.heappop(low))
        elif balance < 0:
            heapq.heappush(low, -heapq.heappop(high))
        prune(low, -1)
        prune(high, 1)
        result.append(median())
    return result


if __name__ == "__main__":
    assert median_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3) == [1, -1, -1, 3, 5, 6]
    assert median_sliding_window([1, 2, 3, 4, 2, 3, 1, 4, 2], 3) == [2, 3, 3, 3, 2, 3, 2]
    assert median_sliding_window([1, 4, 2, 3], 4) == [2.5]
    assert median_sliding_window([5, 5, 5, 1, 1], 2) == [5, 5, 3, 1]
    print("ok")
