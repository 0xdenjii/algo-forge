# Classic - Rabin-Karp String Search
# Problem: Find all occurrences of pattern p in text t.
# Approach: Polynomial rolling hash mod a large prime; verify on hash hit
#   to rule out collisions.
# Time: O(n + m) expected  Space: O(1) extra (plus output)

BASE, MOD = 911382323, (1 << 61) - 1


def rabin_karp(t: str, p: str) -> list[int]:
    n, m = len(t), len(p)
    if m == 0:
        return list(range(n + 1))
    if m > n:
        return []
    hp = ht = 0
    for i in range(m):
        hp = (hp * BASE + ord(p[i])) % MOD
        ht = (ht * BASE + ord(t[i])) % MOD
    power = pow(BASE, m - 1, MOD)
    res = []
    for i in range(n - m + 1):
        if hp == ht and t[i:i + m] == p:
            res.append(i)
        if i + m < n:
            ht = ((ht - ord(t[i]) * power) * BASE + ord(t[i + m])) % MOD
    return res


if __name__ == "__main__":
    assert rabin_karp("abracadabra", "abra") == [0, 7]
    assert rabin_karp("aaaa", "aa") == [0, 1, 2]
    assert rabin_karp("abc", "d") == []
    print("ok")
