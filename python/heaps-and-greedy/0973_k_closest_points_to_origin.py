"""LeetCode 973 - K Closest Points to Origin

Given points on the plane, return the k points closest to (0, 0) by
Euclidean distance, in any order.

Approach: heapq.nsmallest keyed on squared distance (no sqrt needed).
Time: O(n log k)  Space: O(k)
"""
import heapq


def k_closest(points: list[list[int]], k: int) -> list[list[int]]:
    return heapq.nsmallest(k, points, key=lambda p: p[0] * p[0] + p[1] * p[1])


if __name__ == "__main__":
    assert k_closest([[1, 3], [-2, 2]], 1) == [[-2, 2]]
    assert sorted(k_closest([[3, 3], [5, -1], [-2, 4]], 2)) == [[-2, 4], [3, 3]]
    print("ok")
