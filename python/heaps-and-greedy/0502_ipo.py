"""LeetCode 502 - IPO

Starting with capital w, complete at most k projects. Project i needs
capital[i] to start and adds profits[i]. Maximise the final capital.

Approach: sort projects by required capital; each round, push all now
affordable profits into a max-heap and take the best one.
Time: O(n log n + k log n)  Space: O(n)
"""
import heapq


def find_maximized_capital(k: int, w: int, profits: list[int], capital: list[int]) -> int:
    projects = sorted(zip(capital, profits))
    available: list[int] = []
    i = 0
    for _ in range(k):
        while i < len(projects) and projects[i][0] <= w:
            heapq.heappush(available, -projects[i][1])
            i += 1
        if not available:
            break
        w -= heapq.heappop(available)
    return w


if __name__ == "__main__":
    assert find_maximized_capital(2, 0, [1, 2, 3], [0, 1, 1]) == 4
    assert find_maximized_capital(3, 0, [1, 2, 3], [0, 1, 2]) == 6
    print("ok")
