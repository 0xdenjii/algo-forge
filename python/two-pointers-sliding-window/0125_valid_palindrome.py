"""
Problem: LeetCode 125
Title: 125. Valid Palindrome

Statement:
    Return True if s reads the same forward and backward after lowercasing and
    removing non-alphanumeric characters.

Approach:
    Pointers from both ends skip junk and compare case-insensitively.

Complexity: O(n) time, O(1) space.
"""

def is_palindrome(s: str) -> bool:
    lo, hi = 0, len(s) - 1
    while lo < hi:
        if not s[lo].isalnum():
            lo += 1
        elif not s[hi].isalnum():
            hi -= 1
        else:
            if s[lo].lower() != s[hi].lower():
                return False
            lo += 1
            hi -= 1
    return True


if __name__ == "__main__":
    assert is_palindrome("A man, a plan, a canal: Panama")
    assert not is_palindrome("race a car")
    assert is_palindrome(" ")
    print("ok")
