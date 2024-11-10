# Classic - Z-Algorithm
# Problem: Compute z[i] = length of the longest common prefix of s and
#   s[i:]; use it to find pattern occurrences via pattern + sep + text.
# Approach: Maintain the rightmost match window [l, r) and reuse values.
# Time: O(n)  Space: O(n)


def z_function(s: str) -> list[int]:
    n = len(s)
    z = [0] * n
    if n:
        z[0] = n
    l = r = 0
    for i in range(1, n):
        if i < r:
            z[i] = min(r - i, z[i - l])
        while i + z[i] < n and s[z[i]] == s[i + z[i]]:
            z[i] += 1
        if i + z[i] > r:
            l, r = i, i + z[i]
    return z


def find_all(text: str, pat: str) -> list[int]:
    if not pat:
        return list(range(len(text) + 1))
    z = z_function(pat + "\0" + text)
    m = len(pat)
    return [i - m - 1 for i in range(m + 1, len(z)) if z[i] >= m]


if __name__ == "__main__":
    assert z_function("aaabaab") == [7, 2, 1, 0, 2, 1, 0]
    assert find_all("abacaba", "aba") == [0, 4]
    print("ok")
