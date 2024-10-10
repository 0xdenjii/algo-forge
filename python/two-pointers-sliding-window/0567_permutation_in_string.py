"""
Problem: LeetCode 567
Title: 567. Permutation in String

Statement:
    Return True if s2 contains a permutation of s1 (lowercase letters) as a
    substring.

Approach:
    Maintain a count difference array and how many letters have difference 0;
    26 zeros means a match.

Complexity: O(|s2|) time, O(1) space.
"""

def check_inclusion(s1: str, s2: str) -> bool:
    m = len(s1)
    if m > len(s2):
        return False
    diff = [0] * 26
    for a, b in zip(s1, s2):
        diff[ord(a) - 97] += 1
        diff[ord(b) - 97] -= 1
    zeros = diff.count(0)
    if zeros == 26:
        return True

    def bump(idx: int, delta: int) -> int:
        before = diff[idx] == 0
        diff[idx] += delta
        return (diff[idx] == 0) - before

    for i in range(m, len(s2)):
        zeros += bump(ord(s2[i]) - 97, -1)
        zeros += bump(ord(s2[i - m]) - 97, +1)
        if zeros == 26:
            return True
    return False


if __name__ == "__main__":
    assert check_inclusion("ab", "eidbaooo")
    assert not check_inclusion("ab", "eidboaoo")
    assert check_inclusion("adc", "dcda")
    print("ok")
