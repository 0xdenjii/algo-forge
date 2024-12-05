"""
LeetCode 51 - N-Queens
Problem: Place n queens on an n x n board so that no two attack each other.
Return all distinct board configurations ('Q' and '.').
Approach: Row-by-row placement; track used columns and both diagonals
(r - c and r + c) in sets for O(1) conflict checks.
Time: O(n!)  Space: O(n)
"""
from typing import List


def solve_n_queens(n: int) -> List[List[str]]:
    cols, diag, anti = set(), set(), set()
    placement: List[int] = []
    out: List[List[str]] = []

    def dfs(r: int) -> None:
        if r == n:
            out.append(["." * c + "Q" + "." * (n - c - 1) for c in placement])
            return
        for c in range(n):
            if c in cols or r - c in diag or r + c in anti:
                continue
            cols.add(c)
            diag.add(r - c)
            anti.add(r + c)
            placement.append(c)
            dfs(r + 1)
            placement.pop()
            cols.remove(c)
            diag.remove(r - c)
            anti.remove(r + c)

    dfs(0)
    return out


if __name__ == "__main__":
    sols = solve_n_queens(4)
    assert sols == [[".Q..", "...Q", "Q...", "..Q."], ["..Q.", "Q...", "...Q", ".Q.."]]
    assert len(solve_n_queens(8)) == 92
    print("\n".join(sols[0]))
