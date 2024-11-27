# LeetCode 994 - Rotting Oranges
# Grid cells: 0 empty, 1 fresh, 2 rotten. Each minute, fresh oranges adjacent to rotten ones rot.
# Return the minimum minutes until no fresh orange remains, or -1 if impossible.
# Approach: multi-source BFS from all rotten oranges, level by level.
# Time: O(m*n)  Space: O(m*n)

from collections import deque


def oranges_rotting(grid: list[list[int]]) -> int:
    g = [row[:] for row in grid]
    m, n = len(g), len(g[0])
    q = deque()
    fresh = 0
    for i in range(m):
        for j in range(n):
            if g[i][j] == 2:
                q.append((i, j))
            elif g[i][j] == 1:
                fresh += 1
    minutes = 0
    while q and fresh:
        minutes += 1
        for _ in range(len(q)):
            r, c = q.popleft()
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and g[nr][nc] == 1:
                    g[nr][nc] = 2
                    fresh -= 1
                    q.append((nr, nc))
    return -1 if fresh else minutes


if __name__ == "__main__":
    assert oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]) == 4
    assert oranges_rotting([[2, 1, 1], [0, 1, 1], [1, 0, 1]]) == -1
    assert oranges_rotting([[0, 2]]) == 0
    print("minutes:", oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]))
