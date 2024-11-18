# LeetCode 207 - Course Schedule
# There are numCourses courses and pairs [a, b] meaning b must be taken before a.
# Return True if it is possible to finish all courses.
# Approach: Kahn's topological sort; if every node gets popped, there is no cycle.
# Time: O(V + E)  Space: O(V + E)

from collections import deque


def can_finish(num_courses: int, prerequisites: list[list[int]]) -> bool:
    adj = [[] for _ in range(num_courses)]
    indeg = [0] * num_courses
    for a, b in prerequisites:
        adj[b].append(a)
        indeg[a] += 1
    q = deque(i for i in range(num_courses) if indeg[i] == 0)
    done = 0
    while q:
        u = q.popleft()
        done += 1
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return done == num_courses


if __name__ == "__main__":
    assert can_finish(2, [[1, 0]])
    assert not can_finish(2, [[1, 0], [0, 1]])
    print("can_finish(4, chain):", can_finish(4, [[1, 0], [2, 1], [3, 2]]))
