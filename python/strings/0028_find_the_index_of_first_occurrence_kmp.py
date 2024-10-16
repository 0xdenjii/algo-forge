# LeetCode 28 - Find the Index of the First Occurrence in a String
# Problem: Return the first index of needle in haystack, or -1.
# Approach: KMP - build the prefix (failure) function of needle, then scan
#   haystack without ever moving backwards.
# Time: O(n + m)  Space: O(m)


def prefix_function(p: str) -> list[int]:
    pi = [0] * len(p)
    k = 0
    for i in range(1, len(p)):
        while k and p[i] != p[k]:
            k = pi[k - 1]
        if p[i] == p[k]:
            k += 1
        pi[i] = k
    return pi


def str_str(haystack: str, needle: str) -> int:
    if not needle:
        return 0
    pi = prefix_function(needle)
    k = 0
    for i, ch in enumerate(haystack):
        while k and ch != needle[k]:
            k = pi[k - 1]
        if ch == needle[k]:
            k += 1
            if k == len(needle):
                return i - k + 1
    return -1


if __name__ == "__main__":
    assert str_str("sadbutsad", "sad") == 0
    assert str_str("leetcode", "leeto") == -1
    assert str_str("aaaaab", "aab") == 3
    assert prefix_function("aabaaab") == [0, 1, 0, 1, 2, 2, 3]
    print("ok")
