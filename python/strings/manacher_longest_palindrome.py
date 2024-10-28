# Classic - Manacher's Algorithm
# Problem: Find the longest palindromic substring in linear time.
# Approach: Interleave separators so all palindromes are odd, then mirror
#   radii inside the rightmost known palindrome.
# Time: O(n)  Space: O(n)


def manacher(s: str) -> str:
    if not s:
        return ""
    t = "#" + "#".join(s) + "#"
    n = len(t)
    p = [0] * n
    center = right = 0
    for i in range(n):
        if i < right:
            p[i] = min(right - i, p[2 * center - i])
        while i - p[i] - 1 >= 0 and i + p[i] + 1 < n and t[i - p[i] - 1] == t[i + p[i] + 1]:
            p[i] += 1
        if i + p[i] > right:
            center, right = i, i + p[i]
    best = max(range(n), key=p.__getitem__)
    start = (best - p[best]) // 2
    return s[start:start + p[best]]


if __name__ == "__main__":
    assert manacher("babad") in ("bab", "aba")
    assert manacher("cbbd") == "bb"
    assert manacher("forgeeksskeegfor") == "geeksskeeg"
    print("ok")
