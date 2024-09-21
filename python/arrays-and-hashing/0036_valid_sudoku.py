# Problem: LeetCode 36 - Valid Sudoku
#
# Determine whether a partially filled 9x9 Sudoku board is valid: no digit
# repeats in any row, column or 3x3 box. Empty cells are '.'.
#
# Approach: One pass, recording (kind, index, digit) keys in a single set.
# Complexity: O(81) = O(1) time and space

def is_valid_sudoku(board: list[list[str]]) -> bool:
    seen: set[tuple] = set()
    for r in range(9):
        for c in range(9):
            d = board[r][c]
            if d == ".":
                continue
            keys = (("r", r, d), ("c", c, d), ("b", r // 3, c // 3, d))
            if any(k in seen for k in keys):
                return False
            seen.update(keys)
    return True


if __name__ == "__main__":
    board = [list(row) for row in [
        "53..7....", "6..195...", ".98....6.", "8...6...3", "4..8.3..1",
        "7...2...6", ".6....28.", "...419..5", "....8..79"]]
    assert is_valid_sudoku(board)
    board[0][0] = "8"
    assert not is_valid_sudoku(board)
    print("ok")
