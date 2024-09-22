# Problem: LeetCode 383 - Ransom Note
#
# Return True if ransomNote can be built from the letters of magazine, each
# letter used at most once.
#
# Approach: Counter subtraction: the note's counts must be covered by the magazine's.
# Complexity: O(n + m) time, O(k) space

from collections import Counter


def can_construct(ransom_note: str, magazine: str) -> bool:
    return not (Counter(ransom_note) - Counter(magazine))


if __name__ == "__main__":
    assert not can_construct("a", "b")
    assert not can_construct("aa", "ab")
    assert can_construct("aa", "aab")
    print("ok")
