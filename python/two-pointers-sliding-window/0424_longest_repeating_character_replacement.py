"""
Problem: LeetCode 424
Title: 424. Longest Repeating Character Replacement

Statement:
    You may replace at most k characters of s. Return the length of the longest
    substring containing one repeated letter after replacements.

Approach:
    Track the max frequency seen; the window never needs to shrink below the
    best answer, so it slides instead of shrinking.

Complexity: O(n) time, O(alphabet) space.
"""

from collections import Counter


def character_replacement(s: str, k: int) -> int:
    count: Counter[str] = Counter()
    left = max_freq = 0
    for right, ch in enumerate(s):
        count[ch] += 1
        max_freq = max(max_freq, count[ch])
        if right - left + 1 - max_freq > k:
            count[s[left]] -= 1
            left += 1
    return len(s) - left


if __name__ == "__main__":
    assert character_replacement("ABAB", 2) == 4
    assert character_replacement("AABABBA", 1) == 4
    assert character_replacement("", 3) == 0
    print("ok")
