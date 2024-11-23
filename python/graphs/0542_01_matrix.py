# LeetCode 542 - 01 Matrix
# Given a binary matrix, return a matrix where each cell holds the distance to the nearest 0
# (4-directional moves).
# Approach: multi-source BFS starting from all zero cells simultaneously.
# Time: O(m*n)  Space: O(m*n)

from collections import deque


def update_matrix(mat: list[list[int]]) -> list[list[int]]:
    m, n = len(mat), len(mat[0])
    dist = [[-1] * n for _ in range(m)]
    q = deque()
    for i in range(m):
        for j in range(n):
            if mat[i][j] == 0:
                dist[i][j] = 0
                q.append((i, j))
    while q:
        r, c = q.popleft()
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < m and 0 <= nc < n and dist[nr][nc] == -1:
                dist[nr][nc] = dist[r][c] + 1
                q.append((nr, nc))
    return dist


if __name__ == "__main__":
    assert update_matrix([[0, 0, 0], [0, 1, 0], [1, 1, 1]]) == [[0, 0, 0], [0, 1, 0], [1, 2, 1]]
    print("dist:", update_matrix([[0, 0, 0], [0, 1, 0], [1, 1, 1]]))
