# Problem: LeetCode 318 - Maximum Product of Word Lengths
#
# Return max len(a) * len(b) over word pairs sharing no common letters.
#
# Approach: Encode each word's letter set as a 26-bit mask; disjoint iff masks AND to 0.
#   Keep only the longest word per mask.
# Complexity: O(n^2) time worst case, O(n) space

def max_product(words: list[str]) -> int:
    best_len: dict[int, int] = {}
    for w in words:
        mask = 0
        for c in w:
            mask |= 1 << (ord(c) - 97)
        best_len[mask] = max(best_len.get(mask, 0), len(w))
    items = list(best_len.items())
    best = 0
    for i, (m1, l1) in enumerate(items):
        for m2, l2 in items[i + 1:]:
            if m1 & m2 == 0:
                best = max(best, l1 * l2)
    return best


if __name__ == "__main__":
    assert max_product(["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]) == 16
    assert max_product(["a", "aa", "aaa", "aaaa"]) == 0
    print("ok")
