# LeetCode 130 - Surrounded Regions
# Given a board of 'X' and 'O', flip every 'O' region fully surrounded by 'X' into 'X'.
# Regions touching the border are not captured.
# Approach: mark border-connected 'O's via DFS as safe, then flip the rest.
# Time: O(m*n)  Space: O(m*n)

def solve(board: list[list[str]]) -> None:
    if not board:
        return
    m, n = len(board), len(board[0])
    stack = [(i, j) for i in range(m) for j in (0, n - 1)] + \
            [(i, j) for i in (0, m - 1) for j in range(n)]
    while stack:
        r, c = stack.pop()
        if 0 <= r < m and 0 <= c < n and board[r][c] == "O":
            board[r][c] = "S"
            stack.extend(((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)))
    for i in range(m):
        for j in range(n):
            board[i][j] = "O" if board[i][j] == "S" else "X"


if __name__ == "__main__":
    b = [list("XXXX"), list("XOOX"), list("XXOX"), list("XOXX")]
    solve(b)
    assert ["".join(r) for r in b] == ["XXXX", "XXXX", "XXXX", "XOXX"]
    print("\n".join("".join(r) for r in b))
