"""
LeetCode 1239 - Maximum Length of a Concatenated String with Unique Characters
Problem: Choose a subsequence of strings whose concatenation has all unique
characters; return the maximum possible length.
Approach: Convert each self-unique string to a 26-bit mask, then
include/exclude backtracking while masks stay disjoint.
Time: O(2^n)  Space: O(n)
"""
from typing import List


def max_length(arr: List[str]) -> int:
    masks = []
    for s in arr:
        m = 0
        for ch in s:
            bit = 1 << (ord(ch) - ord("a"))
            if m & bit:
                break
            m |= bit
        else:
            masks.append(m)

    def dfs(i: int, cur: int) -> int:
        if i == len(masks):
            return bin(cur).count("1")
        best = dfs(i + 1, cur)
        if not cur & masks[i]:
            best = max(best, dfs(i + 1, cur | masks[i]))
        return best

    return dfs(0, 0)


if __name__ == "__main__":
    assert max_length(["un", "iq", "ue"]) == 4
    assert max_length(["cha", "r", "act", "ers"]) == 6
    assert max_length(["abcdefghijklmnopqrstuvwxyz"]) == 26
    assert max_length(["aa", "bb"]) == 0
    print("ok")
