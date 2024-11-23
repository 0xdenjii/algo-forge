# LeetCode 778 - Swim in Rising Water
# grid[i][j] is an elevation; at time t you can swim through cells with elevation <= t.
# Return the least time to reach the bottom-right from the top-left.
# Approach: Dijkstra-style min-heap where a path's cost is the max elevation along it.
# Time: O(n^2 log n)  Space: O(n^2)

import heapq


def swim_in_water(grid: list[list[int]]) -> int:
    n = len(grid)
    pq = [(grid[0][0], 0, 0)]
    seen = {(0, 0)}
    while pq:
        t, r, c = heapq.heappop(pq)
        if (r, c) == (n - 1, n - 1):
            return t
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < n and 0 <= nc < n and (nr, nc) not in seen:
                seen.add((nr, nc))
                heapq.heappush(pq, (max(t, grid[nr][nc]), nr, nc))
    return -1


if __name__ == "__main__":
    assert swim_in_water([[0, 2], [1, 3]]) == 3
    g = [[0, 1, 2, 3, 4], [24, 23, 22, 21, 5], [12, 13, 14, 15, 16], [11, 17, 18, 19, 20], [10, 9, 8, 7, 6]]
    assert swim_in_water(g) == 16
    print("time:", swim_in_water(g))
