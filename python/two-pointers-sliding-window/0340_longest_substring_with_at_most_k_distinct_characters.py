"""
Problem: LeetCode 340
Title: 340. Longest Substring with At Most K Distinct Characters

Statement:
    Return the length of the longest substring containing at most k distinct
    characters (problem 159 is the k = 2 case).

Approach:
    Expand right; while distinct count > k, shrink left.

Complexity: O(n) time, O(k) space.
"""

from collections import defaultdict


def longest_k_distinct(s: str, k: int) -> int:
    count: defaultdict[str, int] = defaultdict(int)
    left = best = 0
    for right, ch in enumerate(s):
        count[ch] += 1
        while len(count) > k:
            count[s[left]] -= 1
            if count[s[left]] == 0:
                del count[s[left]]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    assert longest_k_distinct("eceba", 2) == 3
    assert longest_k_distinct("ccaabbb", 2) == 5
    assert longest_k_distinct("aa", 1) == 2
    assert longest_k_distinct("abc", 0) == 0
    print("ok")
