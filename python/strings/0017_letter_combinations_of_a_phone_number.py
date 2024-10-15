# LeetCode 17 - Letter Combinations of a Phone Number
# Problem: Given digits 2-9, return all letter strings they could
#   represent on a phone keypad.
# Approach: Cartesian product of the letter groups.
# Time: O(4^n * n)  Space: O(4^n * n) for output
from itertools import product

KEYS = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl",
        "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}


def letter_combinations(digits: str) -> list[str]:
    if not digits:
        return []
    return ["".join(p) for p in product(*(KEYS[d] for d in digits))]


if __name__ == "__main__":
    assert letter_combinations("23") == ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    assert letter_combinations("") == []
    print("ok")
