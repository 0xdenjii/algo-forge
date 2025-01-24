"""LeetCode 295 - Find Median from Data Stream

Design a structure supporting add_num(num) and find_median() over a stream.

Approach: two heaps - a max-heap (negated) for the lower half and a min-heap
for the upper half, kept so len(low) - len(high) is 0 or 1.
Time: O(log n) add, O(1) median  Space: O(n)
"""
import heapq


class MedianFinder:
    def __init__(self) -> None:
        self.low: list[int] = []   # max-heap via negation
        self.high: list[int] = []  # min-heap

    def add_num(self, num: int) -> None:
        heapq.heappush(self.low, -num)
        heapq.heappush(self.high, -heapq.heappop(self.low))
        if len(self.high) > len(self.low):
            heapq.heappush(self.low, -heapq.heappop(self.high))

    def find_median(self) -> float:
        if len(self.low) > len(self.high):
            return float(-self.low[0])
        return (-self.low[0] + self.high[0]) / 2


if __name__ == "__main__":
    mf = MedianFinder()
    mf.add_num(1)
    mf.add_num(2)
    assert mf.find_median() == 1.5
    mf.add_num(3)
    assert mf.find_median() == 2.0
    print("ok")
