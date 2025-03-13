"""LeetCode 1046 - Last Stone Weight

Repeatedly smash the two heaviest stones x <= y: equal stones vanish,
otherwise a stone of weight y - x remains. Return the last weight, or 0.

Approach: simulate with a max-heap (negated values).
Time: O(n log n)  Space: O(n)
"""
import heapq


def last_stone_weight(stones: list[int]) -> int:
    heap = [-s for s in stones]
    heapq.heapify(heap)
    while len(heap) > 1:
        y = -heapq.heappop(heap)
        x = -heapq.heappop(heap)
        if y != x:
            heapq.heappush(heap, x - y)
    return -heap[0] if heap else 0


if __name__ == "__main__":
    assert last_stone_weight([2, 7, 4, 1, 8, 1]) == 1
    assert last_stone_weight([1]) == 1
    assert last_stone_weight([3, 3]) == 0
    print("ok")
