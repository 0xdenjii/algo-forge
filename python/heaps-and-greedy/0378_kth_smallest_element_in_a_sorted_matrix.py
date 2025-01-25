"""LeetCode 378 - Kth Smallest Element in a Sorted Matrix

In an n x n matrix whose rows and columns are sorted ascending, return the
kth smallest element.

Approach: k-way merge of the rows with a min-heap; pop k - 1 times.
Time: O(k log n)  Space: O(n)
"""
import heapq


def kth_smallest(matrix: list[list[int]], k: int) -> int:
    n = len(matrix)
    heap = [(matrix[r][0], r, 0) for r in range(min(n, k))]
    heapq.heapify(heap)
    for _ in range(k - 1):
        _, r, c = heapq.heappop(heap)
        if c + 1 < n:
            heapq.heappush(heap, (matrix[r][c + 1], r, c + 1))
    return heap[0][0]


if __name__ == "__main__":
    assert kth_smallest([[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8) == 13
    assert kth_smallest([[-5]], 1) == -5
    print("ok")
