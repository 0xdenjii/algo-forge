# LeetCode 797 - All Paths From Source to Target
# Given a DAG of n nodes as an adjacency list, return all paths from node 0 to node n-1.
# Approach: DFS backtracking; no visited set needed because the graph is acyclic.
# Time: O(2^n * n)  Space: O(n) recursion (excluding output)

def all_paths_source_target(graph: list[list[int]]) -> list[list[int]]:
    target = len(graph) - 1
    out, path = [], [0]

    def dfs(u):
        if u == target:
            out.append(path[:])
            return
        for v in graph[u]:
            path.append(v)
            dfs(v)
            path.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    assert all_paths_source_target([[1, 2], [3], [3], []]) == [[0, 1, 3], [0, 2, 3]]
    print("paths:", all_paths_source_target([[4, 3, 1], [3, 2, 4], [3], [4], []]))
