# Problem: LeetCode 336 - Palindrome Pairs
#
# Return all index pairs (i, j), i != j, such that words[i] + words[j] is a palindrome.
#
# Approach: Hash map of reversed words; for each split of a word into prefix/suffix, if
#   one part is a palindrome look up the reverse of the other.
# Complexity: O(n k^2) time, O(n k) space

def palindrome_pairs(words: list[str]) -> list[list[int]]:
    index = {w[::-1]: i for i, w in enumerate(words)}
    res = []
    for i, w in enumerate(words):
        for k in range(len(w) + 1):
            pre, suf = w[:k], w[k:]
            if pre == pre[::-1] and suf in index and index[suf] != i:
                res.append([index[suf], i])
            if k != len(w) and suf == suf[::-1] and pre in index and index[pre] != i:
                res.append([i, index[pre]])
    return res


if __name__ == "__main__":
    out = palindrome_pairs(["abcd", "dcba", "lls", "s", "sssll"])
    assert sorted(map(tuple, out)) == sorted([(0, 1), (1, 0), (3, 2), (2, 4)])
    assert sorted(map(tuple, palindrome_pairs(["a", ""]))) == [(0, 1), (1, 0)]
    print("ok")
