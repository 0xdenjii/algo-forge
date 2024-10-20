# LeetCode 242 - Valid Anagram
# Problem: Is t an anagram of s?
# Approach: Compare character multisets with Counter.
# Time: O(n)  Space: O(alphabet)
from collections import Counter


def is_anagram(s: str, t: str) -> bool:
    return len(s) == len(t) and Counter(s) == Counter(t)


if __name__ == "__main__":
    assert is_anagram("anagram", "nagaram")
    assert not is_anagram("rat", "car")
    print("ok")
