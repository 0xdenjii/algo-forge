"""
LeetCode 17 - Letter Combinations of a Phone Number
Problem: Given a string of digits 2-9, return all letter combinations the
number could represent using the classic phone keypad mapping.
Approach: DFS over digit positions, appending each mapped letter.
Time: O(4^n * n)  Space: O(n) recursion (excluding output)
"""
from typing import List

KEYPAD = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl",
          "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}


def letter_combinations(digits: str) -> List[str]:
    if not digits:
        return []
    out: List[str] = []
    path: List[str] = []

    def dfs(i: int) -> None:
        if i == len(digits):
            out.append("".join(path))
            return
        for ch in KEYPAD[digits[i]]:
            path.append(ch)
            dfs(i + 1)
            path.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    assert letter_combinations("23") == ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    assert letter_combinations("") == []
    print(letter_combinations("79")[:5])
