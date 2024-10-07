"""
Problem: LeetCode 76
Title: 76. Minimum Window Substring

Statement:
    Given strings s and t, return the smallest substring of s containing every
    character of t (with multiplicity), or an empty string if none exists.

Approach:
    Expand right, decrementing needs; when all satisfied, shrink left as far as
    possible, recording the best window.

Complexity: O(|s| + |t|) time, O(alphabet) space.
"""

from collections import Counter


def min_window(s: str, t: str) -> str:
    if not t:
        return ""
    need = Counter(t)
    missing = len(t)
    left = 0
    best_start, best_len = 0, len(s) + 1
    for right, ch in enumerate(s):
        if need[ch] > 0:
            missing -= 1
        need[ch] -= 1
        if missing == 0:
            while need[s[left]] < 0:
                need[s[left]] += 1
                left += 1
            if right - left + 1 < best_len:
                best_start, best_len = left, right - left + 1
            need[s[left]] += 1
            missing += 1
            left += 1
    return "" if best_len > len(s) else s[best_start:best_start + best_len]


if __name__ == "__main__":
    assert min_window("ADOBECODEBANC", "ABC") == "BANC"
    assert min_window("a", "a") == "a"
    assert min_window("a", "aa") == ""
    print("ok")
