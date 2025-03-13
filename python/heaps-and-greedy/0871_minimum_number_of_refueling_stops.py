"""LeetCode 871 - Minimum Number of Refueling Stops

A car with startFuel travels to target; stations[i] = [position, fuel].
One unit of fuel per mile. Return the minimum stops to reach target, or -1.

Approach: drive as far as possible; every passed station's fuel goes into a
max-heap. When stuck, retroactively refuel at the biggest passed station.
Time: O(n log n)  Space: O(n)
"""
import heapq


def min_refuel_stops(target: int, start_fuel: int, stations: list[list[int]]) -> int:
    heap: list[int] = []
    reach, stops, i = start_fuel, 0, 0
    while reach < target:
        while i < len(stations) and stations[i][0] <= reach:
            heapq.heappush(heap, -stations[i][1])
            i += 1
        if not heap:
            return -1
        reach -= heapq.heappop(heap)
        stops += 1
    return stops


if __name__ == "__main__":
    assert min_refuel_stops(1, 1, []) == 0
    assert min_refuel_stops(100, 1, [[10, 100]]) == -1
    assert min_refuel_stops(100, 10, [[10, 60], [20, 30], [30, 30], [60, 40]]) == 2
    print("ok")
