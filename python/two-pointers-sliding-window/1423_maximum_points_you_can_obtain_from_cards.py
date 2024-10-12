"""
Problem: LeetCode 1423
Title: 1423. Maximum Points You Can Obtain from Cards

Statement:
    Take exactly k cards from either end of a row; maximise the total points.

Approach:
    Equivalent to leaving a contiguous window of n - k cards with minimum sum.

Complexity: O(n) time, O(1) space.
"""

def max_score(card_points: list[int], k: int) -> int:
    n = len(card_points)
    w = n - k
    total = sum(card_points)
    if w == 0:
        return total
    cur = best = sum(card_points[:w])
    for i in range(w, n):
        cur += card_points[i] - card_points[i - w]
        best = min(best, cur)
    return total - best


if __name__ == "__main__":
    assert max_score([1, 2, 3, 4, 5, 6, 1], 3) == 12
    assert max_score([2, 2, 2], 2) == 4
    assert max_score([9, 7, 7, 9, 7, 7, 9], 7) == 55
    print("ok")
