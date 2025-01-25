"""LeetCode 452 - Minimum Number of Arrows to Burst Balloons

Balloons are horizontal intervals [start, end]; a vertical arrow at x bursts
every balloon with start <= x <= end. Return the minimum arrows needed.

Approach: sort by end and shoot at each uncovered balloon's end point.
Time: O(n log n)  Space: O(1) extra
"""


def find_min_arrow_shots(points: list[list[int]]) -> int:
    arrows, x = 0, float("-inf")
    for s, e in sorted(points, key=lambda p: p[1]):
        if s > x:
            arrows += 1
            x = e
    return arrows


if __name__ == "__main__":
    assert find_min_arrow_shots([[10, 16], [2, 8], [1, 6], [7, 12]]) == 2
    assert find_min_arrow_shots([[1, 2], [3, 4], [5, 6], [7, 8]]) == 4
    assert find_min_arrow_shots([[1, 2], [2, 3], [3, 4], [4, 5]]) == 2
    print("ok")
