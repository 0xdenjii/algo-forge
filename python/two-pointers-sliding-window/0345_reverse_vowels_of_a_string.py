"""
Problem: LeetCode 345
Title: 345. Reverse Vowels of a String (with 344)

Statement:
    Reverse a list of characters in place (344); reverse only the vowels of a
    string (345).

Approach:
    Converging swap; for vowels, skip non-vowels on each side.

Complexity: O(n) time, O(1) extra space (O(n) for the char list in 345).
"""

VOWELS = set("aeiouAEIOU")


def reverse_string(s: list[str]) -> None:
    lo, hi = 0, len(s) - 1
    while lo < hi:
        s[lo], s[hi] = s[hi], s[lo]
        lo += 1
        hi -= 1


def reverse_vowels(s: str) -> str:
    chars = list(s)
    lo, hi = 0, len(chars) - 1
    while lo < hi:
        if chars[lo] not in VOWELS:
            lo += 1
        elif chars[hi] not in VOWELS:
            hi -= 1
        else:
            chars[lo], chars[hi] = chars[hi], chars[lo]
            lo += 1
            hi -= 1
    return "".join(chars)


if __name__ == "__main__":
    a = list("hello")
    reverse_string(a)
    assert a == list("olleh")
    assert reverse_vowels("hello") == "holle"
    assert reverse_vowels("leetcode") == "leotcede"
    print("ok")
