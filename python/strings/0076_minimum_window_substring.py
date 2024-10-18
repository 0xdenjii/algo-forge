# LeetCode 76 - Minimum Window Substring
# Problem: Find the smallest substring of s containing every char of t
#   (with multiplicity); return "" if none.
# Approach: Expand right until all needed, then shrink left while valid.
# Time: O(|s| + |t|)  Space: O(alphabet)
from collections import Counter


def min_window(s: str, t: str) -> str:
    need = Counter(t)
    missing = len(t)
    left = 0
    best_start, best_len = 0, float("inf")
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
    return "" if best_len == float("inf") else s[best_start:best_start + best_len]


if __name__ == "__main__":
    assert min_window("ADOBECODEBANC", "ABC") == "BANC"
    assert min_window("a", "a") == "a"
    assert min_window("a", "aa") == ""
    print("ok")
