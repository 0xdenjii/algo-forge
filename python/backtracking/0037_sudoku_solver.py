"""
LeetCode 37 - Sudoku Solver
Problem: Fill a 9x9 Sudoku board ('.' = empty) in place so that every row,
column and 3x3 box contains digits 1-9 exactly once.
Approach: Bitmask sets for rows/cols/boxes; always branch on the empty cell
with the fewest candidates (MRV heuristic).
Time: exponential worst case, fast in practice  Space: O(81)
"""
from typing import List


def solve_sudoku(board: List[List[str]]) -> None:
    rows = [0] * 9
    cols = [0] * 9
    boxes = [0] * 9
    empty = []
    for r in range(9):
        for c in range(9):
            if board[r][c] == ".":
                empty.append((r, c))
            else:
                bit = 1 << int(board[r][c])
                rows[r] |= bit
                cols[c] |= bit
                boxes[(r // 3) * 3 + c // 3] |= bit

    def candidates(r: int, c: int) -> int:
        used = rows[r] | cols[c] | boxes[(r // 3) * 3 + c // 3]
        return ~used & 0b1111111110

    def dfs() -> bool:
        if not empty:
            return True
        idx = min(range(len(empty)), key=lambda i: bin(candidates(*empty[i])).count("1"))
        empty[idx], empty[-1] = empty[-1], empty[idx]
        r, c = empty.pop()
        b = (r // 3) * 3 + c // 3
        mask = candidates(r, c)
        while mask:
            bit = mask & -mask
            mask ^= bit
            rows[r] |= bit
            cols[c] |= bit
            boxes[b] |= bit
            board[r][c] = str(bit.bit_length() - 1)
            if dfs():
                return True
            rows[r] ^= bit
            cols[c] ^= bit
            boxes[b] ^= bit
        board[r][c] = "."
        empty.append((r, c))
        return False

    dfs()


if __name__ == "__main__":
    puzzle = ["53..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1",
              "7...2...6", ".6....28.", "...419..5", "....8..79"]
    board = [list(row) for row in puzzle]
    solve_sudoku(board)
    for row in board:
        print("".join(row))
    assert "".join(board[0]) == "534678912"
