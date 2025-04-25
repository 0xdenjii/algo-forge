# Problem: LeetCode 212 - Word Search II
#
# Given a letter grid and a word list, return all words that can be formed
# by adjacent (4-dir) cells without reusing a cell.
#
# Approach: Build a trie of words; DFS from every cell following trie edges, pruning
#   exhausted branches once words are found.
# Complexity: O(R C 4 3^(L-1)) time, O(total chars) space

def find_words(board: list[list[str]], words: list[str]) -> list[str]:
    root: dict = {}
    for w in words:
        node = root
        for c in w:
            node = node.setdefault(c, {})
        node["$"] = w
    rows, cols = len(board), len(board[0])
    found: list[str] = []

    def dfs(r: int, c: int, parent: dict) -> None:
        ch = board[r][c]
        node = parent[ch]
        if "$" in node:
            found.append(node.pop("$"))
        board[r][c] = "#"
        for dr, dc in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and board[nr][nc] in node:
                dfs(nr, nc, node)
        board[r][c] = ch
        if not node:
            parent.pop(ch)

    for r in range(rows):
        for c in range(cols):
            if board[r][c] in root:
                dfs(r, c, root)
    return found


if __name__ == "__main__":
    board = [list("oaan"), list("etae"), list("ihkr"), list("iflv")]
    assert sorted(find_words(board, ["oath", "pea", "eat", "rain"])) == ["eat", "oath"]
    print("ok")
