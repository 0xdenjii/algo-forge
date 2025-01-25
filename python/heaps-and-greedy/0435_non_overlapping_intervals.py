"""LeetCode 435 - Non-overlapping Intervals

Return the minimum number of intervals to remove so the rest do not overlap
(touching endpoints are fine).

Approach: activity selection - sort by end, keep every interval that starts
at or after the last kept end; removals = n - kept.
Time: O(n log n)  Space: O(1) extra
"""


def erase_overlap_intervals(intervals: list[list[int]]) -> int:
    kept, end = 0, float("-inf")
    for s, e in sorted(intervals, key=lambda iv: iv[1]):
        if s >= end:
            kept += 1
            end = e
    return len(intervals) - kept


if __name__ == "__main__":
    assert erase_overlap_intervals([[1, 2], [2, 3], [3, 4], [1, 3]]) == 1
    assert erase_overlap_intervals([[1, 2], [1, 2], [1, 2]]) == 2
    assert erase_overlap_intervals([[1, 2], [2, 3]]) == 0
    print("ok")
