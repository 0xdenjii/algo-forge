"""
LeetCode 212 - Word Search II
Problem: Given a character grid and a list of words, return all words that can
be formed from sequentially adjacent cells (each cell used once per word).
Approach: Build a trie; DFS from each cell walking the trie; store the word
at terminal nodes, pop it once found, and prune exhausted trie branches.
Time: O(m * n * 4 * 3^(L-1))  Space: O(total word length)
"""
from typing import List


def find_words(board: List[List[str]], words: List[str]) -> List[str]:
    root: dict = {}
    for w in words:
        node = root
        for ch in w:
            node = node.setdefault(ch, {})
        node["$"] = w

    m, n = len(board), len(board[0])
    found: List[str] = []

    def dfs(r: int, c: int, parent: dict) -> None:
        ch = board[r][c]
        node = parent[ch]
        word = node.pop("$", None)
        if word:
            found.append(word)
        board[r][c] = "#"
        for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < m and 0 <= nc < n and board[nr][nc] in node:
                dfs(nr, nc, node)
        board[r][c] = ch
        if not node:
            parent.pop(ch)

    for r in range(m):
        for c in range(n):
            if board[r][c] in root:
                dfs(r, c, root)
    return found


if __name__ == "__main__":
    b = [list("oaan"), list("etae"), list("ihkr"), list("iflv")]
    assert sorted(find_words(b, ["oath", "pea", "eat", "rain"])) == ["eat", "oath"]
    assert find_words([list("ab"), list("cd")], ["abcb"]) == []
    print("ok")
