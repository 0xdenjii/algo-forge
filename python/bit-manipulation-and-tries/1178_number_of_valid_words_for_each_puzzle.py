# Problem: LeetCode 1178 - Number of Valid Words for Each Puzzle
#
# A word is valid for a 7-letter puzzle if it contains the puzzle's first letter
# and all its letters are in the puzzle. Count valid words per puzzle.
#
# Approach: Count word bitmasks; for each puzzle enumerate the 2^6 submasks of the other
#   letters, always OR-ing in the first letter.
# Complexity: O(W L + P 64) time, O(W) space

from collections import Counter


def find_num_of_valid_words(words: list[str], puzzles: list[str]) -> list[int]:
    def mask_of(s: str) -> int:
        m = 0
        for c in s:
            m |= 1 << (ord(c) - 97)
        return m

    freq = Counter(mask_of(w) for w in words)
    res = []
    for p in puzzles:
        first = 1 << (ord(p[0]) - 97)
        rest = mask_of(p[1:]) & ~first
        sub, total = rest, 0
        while True:
            total += freq.get(sub | first, 0)
            if sub == 0:
                break
            sub = (sub - 1) & rest
        res.append(total)
    return res


if __name__ == "__main__":
    words = ["aaaa", "asas", "able", "ability", "actt", "actor", "access"]
    puzzles = ["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"]
    assert find_num_of_valid_words(words, puzzles) == [1, 1, 3, 2, 4, 0]
    print("ok")
