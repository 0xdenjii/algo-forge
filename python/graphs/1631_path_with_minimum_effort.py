# LeetCode 1631 - Path With Minimum Effort
# A route's effort is the maximum absolute height difference between consecutive cells.
# Return the minimum effort to go from top-left to bottom-right.
# Approach: Dijkstra where the distance is the max edge weight seen so far (minimax path).
# Time: O(m*n log(m*n))  Space: O(m*n)

import heapq


def minimum_effort_path(heights: list[list[int]]) -> int:
    m, n = len(heights), len(heights[0])
    best = [[float("inf")] * n for _ in range(m)]
    best[0][0] = 0
    pq = [(0, 0, 0)]
    while pq:
        e, r, c = heapq.heappop(pq)
        if (r, c) == (m - 1, n - 1):
            return e
        if e > best[r][c]:
            continue
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < m and 0 <= nc < n:
                ne = max(e, abs(heights[nr][nc] - heights[r][c]))
                if ne < best[nr][nc]:
                    best[nr][nc] = ne
                    heapq.heappush(pq, (ne, nr, nc))
    return 0


if __name__ == "__main__":
    assert minimum_effort_path([[1, 2, 2], [3, 8, 2], [5, 3, 5]]) == 2
    assert minimum_effort_path([[1, 2, 3], [3, 8, 4], [5, 3, 5]]) == 1
    print("effort:", minimum_effort_path([[1, 2, 2], [3, 8, 2], [5, 3, 5]]))
