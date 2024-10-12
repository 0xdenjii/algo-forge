"""
Problem: classic (no LeetCode number)
Title: Smallest Difference Pair Across Two Arrays

Statement:
    Given two non-empty integer arrays, find a pair (a from A, b from B) with the
    smallest |a - b|.

Approach:
    Sort both; advance the pointer at the smaller value, the only move that can
    shrink the gap.

Complexity: O(n log n + m log m) time, O(1) extra space.
"""

def smallest_difference(a: list[int], b: list[int]) -> tuple[int, int]:
    a, b = sorted(a), sorted(b)
    i = j = 0
    best = (a[0], b[0])
    while i < len(a) and j < len(b):
        if abs(a[i] - b[j]) < abs(best[0] - best[1]):
            best = (a[i], b[j])
        if a[i] < b[j]:
            i += 1
        elif a[i] > b[j]:
            j += 1
        else:
            break
    return best


if __name__ == "__main__":
    assert smallest_difference([-1, 5, 10, 20, 28, 3], [26, 134, 135, 15, 17]) == (28, 26)
    assert smallest_difference([1, 2], [2, 9]) == (2, 2)
    print("ok")
