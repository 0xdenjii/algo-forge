"""LeetCode 857 - Minimum Cost to Hire K Workers

Hire exactly k workers. Pay must be proportional to quality within the group
and each worker gets at least wage[i]. Return the minimum total pay.

Approach: a group pays ratio * sum(quality), ratio = max wage/quality in it.
Sort by ratio; for each captain ratio keep the k smallest qualities in a
max-heap and evaluate ratio * quality_sum.
Time: O(n log n)  Space: O(n)
"""
import heapq


def mincost_to_hire_workers(quality: list[int], wage: list[int], k: int) -> float:
    workers = sorted((w / q, q) for q, w in zip(quality, wage))
    heap: list[int] = []
    qsum = 0
    best = float("inf")
    for ratio, q in workers:
        heapq.heappush(heap, -q)
        qsum += q
        if len(heap) > k:
            qsum += heapq.heappop(heap)
        if len(heap) == k:
            best = min(best, ratio * qsum)
    return best


if __name__ == "__main__":
    assert abs(mincost_to_hire_workers([10, 20, 5], [70, 50, 30], 2) - 105.0) < 1e-6
    assert abs(mincost_to_hire_workers([3, 1, 10, 10, 1], [4, 8, 2, 2, 7], 3) - 30.66667) < 1e-4
    print("ok")
