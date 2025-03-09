"""LeetCode 703 - Kth Largest Element in a Stream

Design a class that returns the kth largest element after each insertion
into a stream of integers.

Approach: maintain a min-heap holding the k largest values seen so far.
Time: O(log k) per add  Space: O(k)
"""
import heapq


class KthLargest:
    def __init__(self, k: int, nums: list[int]) -> None:
        self.k = k
        self.heap: list[int] = []
        for x in nums:
            self.add(x)

    def add(self, val: int) -> int:
        if len(self.heap) < self.k:
            heapq.heappush(self.heap, val)
        elif val > self.heap[0]:
            heapq.heapreplace(self.heap, val)
        return self.heap[0]


if __name__ == "__main__":
    kl = KthLargest(3, [4, 5, 8, 2])
    assert [kl.add(v) for v in (3, 5, 10, 9, 4)] == [4, 5, 5, 8, 8]
    print("ok")
