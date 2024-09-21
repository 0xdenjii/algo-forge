# Problem: LeetCode 242 - Valid Anagram
#
# Given strings s and t, return True if t is an anagram of s.
#
# Approach: Compare character frequency counters.
# Complexity: O(n) time, O(k) space for alphabet size k

from collections import Counter


def is_anagram(s: str, t: str) -> bool:
    return len(s) == len(t) and Counter(s) == Counter(t)


if __name__ == "__main__":
    assert is_anagram("anagram", "nagaram")
    assert not is_anagram("rat", "car")
    print("ok")
