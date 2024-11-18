# LeetCode 210 - Course Schedule II
# Same input as Course Schedule; return any valid order to take all courses,
# or an empty list if impossible.
# Approach: Kahn's algorithm, recording the pop order.
# Time: O(V + E)  Space: O(V + E)

from collections import deque


def find_order(num_courses: int, prerequisites: list[list[int]]) -> list[int]:
    adj = [[] for _ in range(num_courses)]
    indeg = [0] * num_courses
    for a, b in prerequisites:
        adj[b].append(a)
        indeg[a] += 1
    q = deque(i for i in range(num_courses) if indeg[i] == 0)
    order = []
    while q:
        u = q.popleft()
        order.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return order if len(order) == num_courses else []


if __name__ == "__main__":
    prereq = [[1, 0], [2, 0], [3, 1], [3, 2]]
    order = find_order(4, prereq)
    pos = {c: i for i, c in enumerate(order)}
    assert all(pos[b] < pos[a] for a, b in prereq)
    assert find_order(2, [[0, 1], [1, 0]]) == []
    print("order:", order)
