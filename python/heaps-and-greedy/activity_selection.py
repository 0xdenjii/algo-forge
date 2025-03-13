"""Activity Selection (classic greedy, CLRS 16.1)

Given activities with start and finish times, select a maximum-size set of
mutually compatible activities (an activity may start when another ends).

Approach: sort by finish time and greedily take each activity starting at
or after the last chosen finish. Exchange argument proves optimality.
Time: O(n log n)  Space: O(n) for the result
"""


def select_activities(acts: list[tuple[int, int]]) -> list[tuple[int, int]]:
    chosen: list[tuple[int, int]] = []
    last = float("-inf")
    for s, f in sorted(acts, key=lambda a: a[1]):
        if s >= last:
            chosen.append((s, f))
            last = f
    return chosen


if __name__ == "__main__":
    acts = [(1, 4), (3, 5), (0, 6), (5, 7), (3, 9), (5, 9),
            (6, 10), (8, 11), (8, 12), (2, 14), (12, 16)]
    assert select_activities(acts) == [(1, 4), (5, 7), (8, 11), (12, 16)]
    print("ok")
