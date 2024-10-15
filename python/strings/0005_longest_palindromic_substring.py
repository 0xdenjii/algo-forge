# LeetCode 5 - Longest Palindromic Substring
# Problem: Return the longest substring of s that reads the same forwards
#   and backwards.
# Approach: Expand around all 2n-1 centers (odd and even length).
# Time: O(n^2)  Space: O(1)


def longest_palindrome(s: str) -> str:
    if not s:
        return ""
    lo = hi = 0

    def expand(l: int, r: int) -> tuple[int, int]:
        while l >= 0 and r < len(s) and s[l] == s[r]:
            l -= 1
            r += 1
        return l + 1, r - 1

    for i in range(len(s)):
        for l, r in (expand(i, i), expand(i, i + 1)):
            if r - l > hi - lo:
                lo, hi = l, r
    return s[lo:hi + 1]


if __name__ == "__main__":
    assert longest_palindrome("babad") in ("bab", "aba")
    assert longest_palindrome("cbbd") == "bb"
    assert longest_palindrome("a") == "a"
    print("ok")
