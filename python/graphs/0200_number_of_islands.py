# LeetCode 200 - Number of Islands
# Given an m x n grid of '1' (land) and '0' (water), count the islands.
# An island is formed by connecting adjacent land cells horizontally or vertically.
# Approach: scan the grid; on each unvisited land cell, BFS-flood its island.
# Time: O(m*n)  Space: O(min(m, n)) for the BFS queue (grid copy aside)

from collections import deque


def num_islands(grid: list[list[str]]) -> int:
    if not grid:
        return 0
    g = [row[:] for row in grid]
    m, n = len(g), len(g[0])
    count = 0
    for i in range(m):
        for j in range(n):
            if g[i][j] != "1":
                continue
            count += 1
            g[i][j] = "0"
            q = deque([(i, j)])
            while q:
                r, c = q.popleft()
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < m and 0 <= nc < n and g[nr][nc] == "1":
                        g[nr][nc] = "0"
                        q.append((nr, nc))
    return count


if __name__ == "__main__":
    grid = [list("11000"), list("11000"), list("00100"), list("00011")]
    assert num_islands(grid) == 3
    assert num_islands([list("111"), list("010")]) == 1
    print("num_islands:", num_islands(grid))
