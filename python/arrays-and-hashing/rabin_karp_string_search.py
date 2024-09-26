# Problem: Rabin-Karp String Search
#
# Classic algorithm: return all start indices where pattern occurs in text,
# using a rolling polynomial hash.
#
# Approach: Rolling hash mod a large prime; verify candidate matches to rule out collisions.
# Complexity: O(n + m) expected time, O(1) extra space

def rabin_karp(text: str, pattern: str, base: int = 256, mod: int = 1_000_000_007) -> list[int]:
    n, m = len(text), len(pattern)
    if m == 0:
        return list(range(n + 1))
    if m > n:
        return []
    high = pow(base, m - 1, mod)
    hp = ht = 0
    for i in range(m):
        hp = (hp * base + ord(pattern[i])) % mod
        ht = (ht * base + ord(text[i])) % mod
    out: list[int] = []
    for i in range(n - m + 1):
        if hp == ht and text[i:i + m] == pattern:
            out.append(i)
        if i + m < n:
            ht = ((ht - ord(text[i]) * high) * base + ord(text[i + m])) % mod
    return out


if __name__ == "__main__":
    assert rabin_karp("abracadabra", "abra") == [0, 7]
    assert rabin_karp("aaaa", "aa") == [0, 1, 2]
    assert rabin_karp("abc", "d") == []
    print("ok")
