"""LeetCode 1094 - Car Pooling

A car with given capacity drives east. trips[i] = [passengers, from, to].
Return whether all trips can be served without exceeding capacity.

Approach: sort trips by pickup; a min-heap of (dropoff, passengers) releases
seats before each pickup.
Time: O(n log n)  Space: O(n)
"""
import heapq


def car_pooling(trips: list[list[int]], capacity: int) -> bool:
    onboard: list[tuple[int, int]] = []
    load = 0
    for n, start, end in sorted(trips, key=lambda t: t[1]):
        while onboard and onboard[0][0] <= start:
            load -= heapq.heappop(onboard)[1]
        load += n
        if load > capacity:
            return False
        heapq.heappush(onboard, (end, n))
    return True


if __name__ == "__main__":
    assert not car_pooling([[2, 1, 5], [3, 3, 7]], 4)
    assert car_pooling([[2, 1, 5], [3, 3, 7]], 5)
    assert car_pooling([[2, 1, 5], [3, 5, 7]], 3)
    print("ok")
