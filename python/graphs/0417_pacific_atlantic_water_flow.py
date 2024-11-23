# LeetCode 417 - Pacific Atlantic Water Flow
# Water flows from a cell to neighbors with height <= its own. Pacific touches top/left edges,
# Atlantic touches bottom/right edges. Return cells that can reach both oceans.
# Approach: BFS uphill from each ocean's border cells; intersect the reachable sets.
# Time: O(m*n)  Space: O(m*n)

from collections import deque


def pacific_atlantic(heights: list[list[int]]) -> list[list[int]]:
    m, n = len(heights), len(heights[0])

    def bfs(starts):
        seen = set(starts)
        q = deque(starts)
        while q:
            r, c = q.popleft()
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if (0 <= nr < m and 0 <= nc < n and (nr, nc) not in seen
                        and heights[nr][nc] >= heights[r][c]):
                    seen.add((nr, nc))
                    q.append((nr, nc))
        return seen

    pac = bfs([(0, j) for j in range(n)] + [(i, 0) for i in range(1, m)])
    atl = bfs([(m - 1, j) for j in range(n)] + [(i, n - 1) for i in range(m - 1)])
    return sorted([r, c] for r, c in pac & atl)


if __name__ == "__main__":
    h = [[1, 2, 2, 3, 5], [3, 2, 3, 4, 4], [2, 4, 5, 3, 1], [6, 7, 1, 4, 5], [5, 1, 1, 2, 4]]
    expected = [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
    assert pacific_atlantic(h) == expected
    print("cells:", pacific_atlantic(h))
