"""LeetCode 253 - Meeting Rooms II

Given meeting intervals [start, end), return the minimum number of
conference rooms required.

Approach: sort by start; a min-heap stores end times of rooms in use. Reuse
the earliest-free room when it is free by the new start.
Time: O(n log n)  Space: O(n)
"""
import heapq


def min_meeting_rooms(intervals: list[list[int]]) -> int:
    ends: list[int] = []
    for s, e in sorted(intervals):
        if ends and ends[0] <= s:
            heapq.heapreplace(ends, e)
        else:
            heapq.heappush(ends, e)
    return len(ends)


if __name__ == "__main__":
    assert min_meeting_rooms([[0, 30], [5, 10], [15, 20]]) == 2
    assert min_meeting_rooms([[7, 10], [2, 4]]) == 1
    assert min_meeting_rooms([]) == 0
    print("ok")
