# LeetCode 1091 - Shortest Path in Binary Matrix
# In an n x n binary grid, return the length of the shortest clear path (cells of 0) from
# top-left to bottom-right moving in 8 directions, or -1.
# Approach: BFS from (0,0), counting path length in cells.
# Time: O(n^2)  Space: O(n^2)

from collections import deque


def shortest_path_binary_matrix(grid: list[list[int]]) -> int:
    n = len(grid)
    if grid[0][0] or grid[n - 1][n - 1]:
        return -1
    dist = [[0] * n for _ in range(n)]
    dist[0][0] = 1
    q = deque([(0, 0)])
    while q:
        r, c = q.popleft()
        if (r, c) == (n - 1, n - 1):
            return dist[r][c]
        for dr in (-1, 0, 1):
            for dc in (-1, 0, 1):
                nr, nc = r + dr, c + dc
                if 0 <= nr < n and 0 <= nc < n and not grid[nr][nc] and not dist[nr][nc]:
                    dist[nr][nc] = dist[r][c] + 1
                    q.append((nr, nc))
    return -1


if __name__ == "__main__":
    assert shortest_path_binary_matrix([[0, 1], [1, 0]]) == 2
    assert shortest_path_binary_matrix([[0, 0, 0], [1, 1, 0], [1, 1, 0]]) == 4
    assert shortest_path_binary_matrix([[1, 0, 0], [1, 1, 0], [1, 1, 0]]) == -1
    print("length:", shortest_path_binary_matrix([[0, 0, 0], [1, 1, 0], [1, 1, 0]]))
