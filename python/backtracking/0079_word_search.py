"""
LeetCode 79 - Word Search
Problem: Given an m x n grid of characters and a word, return True if the word
can be built from sequentially adjacent (4-directional) cells, each cell used
at most once.
Approach: DFS from each cell, marking visited cells in place with '#'.
Early exit when the board's letter counts cannot cover the word, and search
from the rarer end of the word first.
Time: O(m * n * 3^L)  Space: O(L)
"""
from collections import Counter
from typing import List


def exist(board: List[List[str]], word: str) -> bool:
    m, n = len(board), len(board[0])
    have = Counter(ch for row in board for ch in row)
    if any(have[c] < k for c, k in Counter(word).items()):
        return False
    if have[word[0]] > have[word[-1]]:
        word = word[::-1]

    def dfs(r: int, c: int, i: int) -> bool:
        if i == len(word):
            return True
        if not (0 <= r < m and 0 <= c < n) or board[r][c] != word[i]:
            return False
        board[r][c] = "#"
        found = (dfs(r + 1, c, i + 1) or dfs(r - 1, c, i + 1)
                 or dfs(r, c + 1, i + 1) or dfs(r, c - 1, i + 1))
        board[r][c] = word[i]
        return found

    return any(dfs(r, c, 0) for r in range(m) for c in range(n))


if __name__ == "__main__":
    g = [list("ABCE"), list("SFCS"), list("ADEE")]
    assert exist(g, "ABCCED")
    assert exist(g, "SEE")
    assert not exist(g, "ABCB")
    print("ok")
