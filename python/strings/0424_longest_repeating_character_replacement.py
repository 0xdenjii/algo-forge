# LeetCode 424 - Longest Repeating Character Replacement
# Problem: With at most k character replacements, find the length of the
#   longest substring consisting of one repeated letter.
# Approach: Window is valid while len - maxFreq <= k; the window never
#   shrinks, so its final size is the answer.
# Time: O(n)  Space: O(alphabet)
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
