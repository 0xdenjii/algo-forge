# Codeforces-style - Smallest String Period
# Problem: Find the shortest string t such that s is t repeated an
#   integer number of times (e.g. "abcabc" -> "abc").
# Approach: With prefix function pi, p = n - pi[n-1] is the smallest
#   period; it is a full repetition iff n % p == 0.
# Time: O(n)  Space: O(n)


def smallest_repeating_unit(s: str) -> str:
    n = len(s)
    if n == 0:
        return ""
    pi = [0] * n
    for i in range(1, n):
        k = pi[i - 1]
        while k and s[i] != s[k]:
            k = pi[k - 1]
        if s[i] == s[k]:
            k += 1
        pi[i] = k
    p = n - pi[-1]
    return s[:p] if n % p == 0 else s


if __name__ == "__main__":
    assert smallest_repeating_unit("abcabc") == "abc"
    assert smallest_repeating_unit("aaaa") == "a"
    assert smallest_repeating_unit("abcab") == "abcab"
    print("ok")
