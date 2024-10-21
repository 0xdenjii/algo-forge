# LeetCode 438 - Find All Anagrams in a String
# Problem: Return all start indices in s where an anagram of p begins.
# Approach: Maintain a 26-count window of length |p| and compare.
# Time: O(26 * n)  Space: O(1)


def find_anagrams(s: str, p: str) -> list[int]:
    m = len(p)
    if m > len(s):
        return []
    need, win = [0] * 26, [0] * 26
    for ch in p:
        need[ord(ch) - 97] += 1
    res = []
    for i, ch in enumerate(s):
        win[ord(ch) - 97] += 1
        if i >= m:
            win[ord(s[i - m]) - 97] -= 1
        if i >= m - 1 and win == need:
            res.append(i - m + 1)
    return res


if __name__ == "__main__":
    assert find_anagrams("cbaebabacd", "abc") == [0, 6]
    assert find_anagrams("abab", "ab") == [0, 1, 2]
    print("ok")
