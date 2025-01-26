"""LeetCode 621 - Task Scheduler

Tasks are letters; identical tasks need at least n intervals between them.
Each interval runs one task or idles. Return the minimum total intervals.

Approach (greedy counting): with max frequency f shared by m tasks, the frame
(f - 1) * (n + 1) + m is a lower bound; the answer is max(frame, len(tasks)).
Time: O(len(tasks))  Space: O(1) (26 letters)
"""
from collections import Counter


def least_interval(tasks: list[str], n: int) -> int:
    counts = Counter(tasks).values()
    f = max(counts)
    m = sum(1 for c in counts if c == f)
    return max(len(tasks), (f - 1) * (n + 1) + m)


if __name__ == "__main__":
    assert least_interval(list("AAABBB"), 2) == 8
    assert least_interval(list("ACABDB"), 1) == 6
    assert least_interval(list("AAABBB"), 3) == 10
    print("ok")
