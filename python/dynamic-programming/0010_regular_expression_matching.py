# LeetCode 10 - Regular Expression Matching
# Problem: Implement full-string matching of s against pattern p supporting
#   '.' (any char) and '*' (zero or more of the preceding element).
# Approach: Memoised recursion on (i, j): first-char match, and for 'x*' either
#   skip the pair or consume one char of s and stay on the pattern.
# Time: O(m * n)  Space: O(m * n)
from functools import cache


def is_match(s: str, p: str) -> bool:
    @cache
    def match(i: int, j: int) -> bool:
        if j == len(p):
            return i == len(s)
        first = i < len(s) and p[j] in (s[i], ".")
        if j + 1 < len(p) and p[j + 1] == "*":
            return match(i, j + 2) or (first and match(i + 1, j))
        return first and match(i + 1, j + 1)

    return match(0, 0)


if __name__ == "__main__":
    assert not is_match("aa", "a")
    assert is_match("aa", "a*")
    assert is_match("ab", ".*")
    assert is_match("aab", "c*a*b")
    assert not is_match("mississippi", "mis*is*p*.")
    print("ok")
