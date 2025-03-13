# LeetCode 5 - Longest Palindromic Substring
# Problem: Return the longest contiguous substring of s that is a palindrome.
# Approach: Expand around each of the 2n-1 centres (the DP table
#   pal[i][j] = s[i]==s[j] and pal[i+1][j-1] collapsed to O(1) space).
# Time: O(n^2)  Space: O(1)


def longest_palindrome(s: str) -> str:
    if not s:
        return ""
    start, end = 0, 0
    for c in range(2 * len(s) - 1):
        l, r = c // 2, c // 2 + c % 2
        while l >= 0 and r < len(s) and s[l] == s[r]:
            l, r = l - 1, r + 1
        if r - l - 1 > end - start:
            start, end = l + 1, r
    return s[start:end]


if __name__ == "__main__":
    assert longest_palindrome("babad") in ("bab", "aba")
    assert longest_palindrome("cbbd") == "bb"
    assert longest_palindrome("a") == "a"
    print("ok")
