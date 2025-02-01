"""LeetCode 630 - Course Schedule III

courses[i] = [duration, lastDay]; courses are taken one at a time starting
on day 1 and must finish by lastDay. Return the maximum number of courses.

Approach: sort by deadline; take each course, and if time exceeds the
deadline drop the longest course taken so far (max-heap of durations).
Time: O(n log n)  Space: O(n)
"""
import heapq


def schedule_course(courses: list[list[int]]) -> int:
    taken: list[int] = []
    time = 0
    for duration, last in sorted(courses, key=lambda c: c[1]):
        heapq.heappush(taken, -duration)
        time += duration
        if time > last:
            time += heapq.heappop(taken)
    return len(taken)


if __name__ == "__main__":
    assert schedule_course([[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]) == 3
    assert schedule_course([[1, 2]]) == 1
    assert schedule_course([[3, 2], [4, 3]]) == 0
    print("ok")
