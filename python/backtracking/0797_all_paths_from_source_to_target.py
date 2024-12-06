"""
LeetCode 797 - All Paths From Source to Target
Problem: Given a DAG of n nodes as an adjacency list, return all paths from
node 0 to node n - 1.
Approach: DFS with a path stack (a DAG needs no visited set).
Time: O(2^n * n)  Space: O(n)
"""
from typing import List


def all_paths_source_target(graph: List[List[int]]) -> List[List[int]]:
    target = len(graph) - 1
    out: List[List[int]] = []
    path = [0]

    def dfs(u: int) -> None:
        if u == target:
            out.append(path.copy())
            return
        for v in graph[u]:
            path.append(v)
            dfs(v)
            path.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    assert all_paths_source_target([[1, 2], [3], [3], []]) == [[0, 1, 3], [0, 2, 3]]
    assert all_paths_source_target([[4, 3, 1], [3, 2, 4], [3], [4], []]) == [
        [0, 4], [0, 3, 4], [0, 1, 3, 4], [0, 1, 2, 3, 4], [0, 1, 4]]
    print("ok")
