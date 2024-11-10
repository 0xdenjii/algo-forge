# Classic - Suffix Array + LCP (Kasai)
# Problem: Sort all suffixes of s and compute LCP of adjacent suffixes;
#   demo: count distinct substrings = n(n+1)/2 - sum(LCP).
# Approach: Prefix doubling, sorting by (rank[i], rank[i+k]); Kasai's
#   algorithm for LCP.
# Time: O(n log^2 n)  Space: O(n)


def suffix_array(s: str) -> list[int]:
    n = len(s)
    if n == 0:
        return []
    rank = [ord(c) for c in s]
    sa = list(range(n))
    k = 1
    while True:
        def key(i: int) -> tuple[int, int]:
            return rank[i], rank[i + k] if i + k < n else -1
        sa.sort(key=key)
        new = [0] * n
        for a, b in zip(sa, sa[1:]):
            new[b] = new[a] + (key(a) != key(b))
        rank = new
        if rank[sa[-1]] == n - 1:
            return sa
        k <<= 1


def lcp_array(s: str, sa: list[int]) -> list[int]:
    n = len(s)
    rank = [0] * n
    for i, p in enumerate(sa):
        rank[p] = i
    lcp, h = [0] * max(n - 1, 0), 0
    for i in range(n):
        if rank[i] > 0:
            j = sa[rank[i] - 1]
            while i + h < n and j + h < n and s[i + h] == s[j + h]:
                h += 1
            lcp[rank[i] - 1] = h
            if h:
                h -= 1
        else:
            h = 0
    return lcp


def distinct_substrings(s: str) -> int:
    n = len(s)
    return n * (n + 1) // 2 - sum(lcp_array(s, suffix_array(s)))


if __name__ == "__main__":
    assert suffix_array("banana") == [5, 3, 1, 0, 4, 2]
    assert lcp_array("banana", [5, 3, 1, 0, 4, 2]) == [1, 3, 0, 0, 2]
    assert distinct_substrings("abab") == 7
    print("ok")
