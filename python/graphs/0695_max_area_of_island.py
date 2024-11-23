# LeetCode 695 - Max Area of Island
# Given a binary grid, return the area (cell count) of the largest 4-connected island,
# or 0 if there is none.
# Approach: iterative DFS from each unvisited land cell, counting cells.
# Time: O(m*n)  Space: O(m*n)

def max_area_of_island(grid: list[list[int]]) -> int:
    m, n = len(grid), len(grid[0])
    seen = [[False] * n for _ in range(m)]
    best = 0
    for i in range(m):
        for j in range(n):
            if grid[i][j] != 1 or seen[i][j]:
                continue
            seen[i][j] = True
            stack, area = [(i, j)], 0
            while stack:
                r, c = stack.pop()
                area += 1
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < m and 0 <= nc < n and grid[nr][nc] == 1 and not seen[nr][nc]:
                        seen[nr][nc] = True
                        stack.append((nr, nc))
            best = max(best, area)
    return best


if __name__ == "__main__":
    g = [[0, 0, 1, 0, 0], [0, 1, 1, 0, 0], [0, 0, 0, 1, 1], [1, 0, 0, 1, 1]]
    assert max_area_of_island(g) == 4
    assert max_area_of_island([[0, 0]]) == 0
    print("max area:", max_area_of_island(g))
