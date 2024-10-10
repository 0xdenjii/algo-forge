"""
Problem: LeetCode 438
Title: 438. Find All Anagrams in a String

Statement:
    Return start indices of all substrings of s that are anagrams of p.

Approach:
    Slide a window of len(p) maintaining counts; compare with p's counts.

Complexity: O(n * alphabet) time, O(alphabet) space.
"""

from collections import Counter


def find_anagrams(s: str, p: str) -> list[int]:
    m = len(p)
    if m > len(s):
        return []
    need = Counter(p)
    window = Counter(s[:m])
    res = [0] if window == need else []
    for i in range(m, len(s)):
        window[s[i]] += 1
        out = s[i - m]
        window[out] -= 1
        if window[out] == 0:
            del window[out]
        if window == need:
            res.append(i - m + 1)
    return res


if __name__ == "__main__":
    assert find_anagrams("cbaebabacd", "abc") == [0, 6]
    assert find_anagrams("abab", "ab") == [0, 1, 2]
    assert find_anagrams("a", "ab") == []
    print("ok")
