"""
Rat in a Maze (classic)
Problem: In an n x n binary maze (1 = open), list every path from (0, 0) to
(n-1, n-1) using moves D, L, R, U without revisiting cells, in lexicographic
order.
Approach: DFS trying directions in alphabetical order, marking visited cells.
Time: O(4^(n^2)) worst  Space: O(n^2)
"""
from typing import List

DIRS = [("D", 1, 0), ("L", 0, -1), ("R", 0, 1), ("U", -1, 0)]


def find_paths(maze: List[List[int]]) -> List[str]:
    n = len(maze)
    if n == 0 or not maze[0][0] or not maze[-1][-1]:
        return []
    seen = [[False] * n for _ in range(n)]
    out: List[str] = []
    path: List[str] = []

    def dfs(r: int, c: int) -> None:
        if (r, c) == (n - 1, n - 1):
            out.append("".join(path))
            return
        seen[r][c] = True
        for name, dr, dc in DIRS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < n and 0 <= nc < n and maze[nr][nc] and not seen[nr][nc]:
                path.append(name)
                dfs(nr, nc)
                path.pop()
        seen[r][c] = False

    dfs(0, 0)
    return out


if __name__ == "__main__":
    maze = [[1, 0, 0, 0], [1, 1, 0, 1], [1, 1, 0, 0], [0, 1, 1, 1]]
    assert find_paths(maze) == ["DDRDRR", "DRDDRR"]
    assert find_paths([[1, 0], [0, 1]]) == []
    print(find_paths(maze))
