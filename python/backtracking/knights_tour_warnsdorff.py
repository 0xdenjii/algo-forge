"""
Knight's Tour (classic)
Problem: Find a sequence of knight moves on an n x n board that visits every
square exactly once, starting from a given square.
Approach: Backtracking with moves ordered by Warnsdorff's heuristic (fewest
onward moves first), which rarely needs to backtrack.
Time: O(8^(n^2)) worst, ~O(n^2) in practice  Space: O(n^2)
"""
from typing import List, Optional, Tuple

MOVES = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)]


def knights_tour(n: int, start: Tuple[int, int] = (0, 0)) -> Optional[List[List[int]]]:
    board = [[-1] * n for _ in range(n)]

    def free_moves(r: int, c: int) -> List[Tuple[int, int]]:
        return [(r + dr, c + dc) for dr, dc in MOVES
                if 0 <= r + dr < n and 0 <= c + dc < n and board[r + dr][c + dc] == -1]

    def dfs(r: int, c: int, step: int) -> bool:
        board[r][c] = step
        if step == n * n - 1:
            return True
        for nr, nc in sorted(free_moves(r, c), key=lambda p: len(free_moves(*p))):
            if dfs(nr, nc, step + 1):
                return True
        board[r][c] = -1
        return False

    return board if dfs(start[0], start[1], 0) else None


def is_valid_tour(board: List[List[int]]) -> bool:
    n = len(board)
    pos = {board[r][c]: (r, c) for r in range(n) for c in range(n)}
    return sorted(pos) == list(range(n * n)) and all(
        (abs(pos[i][0] - pos[i + 1][0]), abs(pos[i][1] - pos[i + 1][1])) in ((1, 2), (2, 1))
        for i in range(n * n - 1))


if __name__ == "__main__":
    b = knights_tour(8)
    assert b is not None and is_valid_tour(b)
    assert knights_tour(3) is None
    for row in b:
        print(" ".join(f"{v:2d}" for v in row))
