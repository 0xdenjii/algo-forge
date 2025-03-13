"""LeetCode 1167 - Minimum Cost to Connect Sticks

Connecting sticks of lengths x and y costs x + y. Return the minimum cost to
connect all sticks into one.

Approach: Huffman-style greedy - always merge the two shortest sticks.
Time: O(n log n)  Space: O(n)
"""
import heapq


def connect_sticks(sticks: list[int]) -> int:
    heap = list(sticks)
    heapq.heapify(heap)
    cost = 0
    while len(heap) > 1:
        s = heapq.heappop(heap) + heapq.heappop(heap)
        cost += s
        heapq.heappush(heap, s)
    return cost


if __name__ == "__main__":
    assert connect_sticks([2, 4, 3]) == 14
    assert connect_sticks([1, 8, 3, 5]) == 30
    assert connect_sticks([5]) == 0
    print("ok")
