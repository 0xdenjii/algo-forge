"""
Problem: LeetCode 392
Title: 392. Is Subsequence

Statement:
    Return True if s is a subsequence of t.

Approach:
    Advance through t; advance the s pointer on each match.

Complexity: O(|t|) time, O(1) space.
"""

def is_subsequence(s: str, t: str) -> bool:
    i = 0
    for ch in t:
        if i < len(s) and s[i] == ch:
            i += 1
    return i == len(s)


if __name__ == "__main__":
    assert is_subsequence("abc", "ahbgdc")
    assert not is_subsequence("axc", "ahbgdc")
    assert is_subsequence("", "x")
    print("ok")
