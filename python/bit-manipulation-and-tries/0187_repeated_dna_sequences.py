# Problem: LeetCode 187 - Repeated DNA Sequences
#
# Return all 10-letter substrings occurring more than once in a DNA string.
#
# Approach: Encode each nucleotide in 2 bits; roll a 20-bit window hash.
# Complexity: O(n) time, O(n) space

def find_repeated_dna_sequences(s: str) -> list[str]:
    code = {"A": 0, "C": 1, "G": 2, "T": 3}
    mask = (1 << 20) - 1
    seen: set[int] = set()
    added: set[int] = set()
    res = []
    h = 0
    for i, c in enumerate(s):
        h = ((h << 2) | code[c]) & mask
        if i >= 9:
            if h in seen and h not in added:
                added.add(h)
                res.append(s[i - 9:i + 1])
            seen.add(h)
    return res


if __name__ == "__main__":
    out = find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT")
    assert sorted(out) == ["AAAAACCCCC", "CCCCCAAAAA"]
    assert find_repeated_dna_sequences("AAAAAAAAAAAAA") == ["AAAAAAAAAA"]
    print("ok")
