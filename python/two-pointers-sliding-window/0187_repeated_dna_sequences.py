"""
Problem: LeetCode 187
Title: 187. Repeated DNA Sequences

Statement:
    Return all 10-letter substrings that occur more than once in a DNA string.

Approach:
    Encode each base in 2 bits; roll a 20-bit hash across the string.

Complexity: O(n) time, O(n) space.
"""

def find_repeated_dna_sequences(s: str) -> list[str]:
    code = {"A": 0, "C": 1, "G": 2, "T": 3}
    mask = (1 << 20) - 1
    seen: set[int] = set()
    added: set[int] = set()
    res = []
    h = 0
    for i, ch in enumerate(s):
        h = ((h << 2) | code[ch]) & mask
        if i >= 9:
            if h in seen and h not in added:
                added.add(h)
                res.append(s[i - 9:i + 1])
            seen.add(h)
    return res


if __name__ == "__main__":
    assert sorted(find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT")) == ["AAAAACCCCC", "CCCCCAAAAA"]
    assert find_repeated_dna_sequences("AAAAAAAAAAAAA") == ["AAAAAAAAAA"]
    print("ok")
