# LeetCode 1584 - Min Cost to Connect All Points
# Given points on a plane, the cost to connect two points is their Manhattan distance.
# Return the minimum cost to connect all points.
# Approach: dense Prim's algorithm with an O(n) array scan per step (complete graph).
# Time: O(n^2)  Space: O(n)

def min_cost_connect_points(points: list[list[int]]) -> int:
    n = len(points)
    INF = float("inf")
    best = [INF] * n
    in_tree = [False] * n
    best[0] = 0
    total = 0
    for _ in range(n):
        u = min((i for i in range(n) if not in_tree[i]), key=best.__getitem__)
        in_tree[u] = True
        total += best[u]
        ux, uy = points[u]
        for v in range(n):
            if not in_tree[v]:
                d = abs(ux - points[v][0]) + abs(uy - points[v][1])
                if d < best[v]:
                    best[v] = d
    return total


if __name__ == "__main__":
    assert min_cost_connect_points([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]) == 20
    assert min_cost_connect_points([[3, 12], [-2, 5], [-4, 1]]) == 18
    print("mst cost:", min_cost_connect_points([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]))
