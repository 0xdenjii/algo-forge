# Problem: LeetCode 387 - First Unique Character in a String
#
# Return the index of the first non-repeating character, or -1.
#
# Approach: Count characters, then scan for the first with count 1.
# Complexity: O(n) time, O(k) space

from collections import Counter


def first_uniq_char(s: str) -> int:
    counts = Counter(s)
    return next((i for i, ch in enumerate(s) if counts[ch] == 1), -1)


if __name__ == "__main__":
    assert first_uniq_char("leetcode") == 0
    assert first_uniq_char("loveleetcode") == 2
    assert first_uniq_char("aabb") == -1
    print("ok")
