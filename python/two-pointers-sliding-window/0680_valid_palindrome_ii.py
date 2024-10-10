"""
Problem: LeetCode 680
Title: 680. Valid Palindrome II

Statement:
    Return True if s can become a palindrome after deleting at most one character.

Approach:
    On the first mismatch, try skipping either side and check the remainder.

Complexity: O(n) time, O(1) space.
"""

def valid_palindrome(s: str) -> bool:
    def is_pal(lo: int, hi: int) -> bool:
        while lo < hi:
            if s[lo] != s[hi]:
                return False
            lo += 1
            hi -= 1
        return True

    lo, hi = 0, len(s) - 1
    while lo < hi:
        if s[lo] != s[hi]:
            return is_pal(lo + 1, hi) or is_pal(lo, hi - 1)
        lo += 1
        hi -= 1
    return True


if __name__ == "__main__":
    assert valid_palindrome("aba")
    assert valid_palindrome("abca")
    assert not valid_palindrome("abc")
    print("ok")
