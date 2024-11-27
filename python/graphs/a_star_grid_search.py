# A* Search on a Grid (classic algorithm)
# Given a grid with walls ('#') and open cells, find the length of the shortest
# 4-directional path from start to goal, or -1.
# Approach: A* with f = g + h, using the admissible Manhattan-distance heuristic.
# Time: O(N log N) worst case for N cells  Space: O(N)

import heapq


def a_star(grid: list[str], start: tuple[int, int], goal: tuple[int, int]) -> int:
    m, n = len(grid), len(grid[0])

    def h(p):
        return abs(p[0] - goal[0]) + abs(p[1] - goal[1])

    g = {start: 0}
    pq = [(h(start), 0, start)]
    while pq:
        _, cost, cur = heapq.heappop(pq)
        if cur == goal:
            return cost
        if cost > g[cur]:
            continue
        r, c = cur
        for nxt in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            nr, nc = nxt
            if 0 <= nr < m and 0 <= nc < n and grid[nr][nc] != "#":
                nc_cost = cost + 1
                if nc_cost < g.get(nxt, float("inf")):
                    g[nxt] = nc_cost
                    heapq.heappush(pq, (nc_cost + h(nxt), nc_cost, nxt))
    return -1


if __name__ == "__main__":
    grid = ["....#",
            ".##.#",
            "...#.",
            "#....",
            "....."]
    assert a_star(grid, (0, 0), (2, 4)) == 8
    assert a_star([".#.", "##.", "..."], (0, 0), (2, 2)) == -1
    print("path length:", a_star(grid, (0, 0), (2, 4)))
