# LeetCode 1192 - Critical Connections in a Network
# Given n servers and undirected connections, return all critical connections: edges whose
# removal disconnects some servers (i.e. bridges).
# Approach: Tarjan's bridge-finding with discovery times and low-links (iterative DFS).
# Time: O(V + E)  Space: O(V + E)

def critical_connections(n: int, connections: list[list[int]]) -> list[list[int]]:
    adj = [[] for _ in range(n)]
    for i, (a, b) in enumerate(connections):
        adj[a].append((b, i))
        adj[b].append((a, i))
    disc = [-1] * n
    low = [0] * n
    bridges, timer = [], 0
    for root in range(n):
        if disc[root] != -1:
            continue
        disc[root] = low[root] = timer
        timer += 1
        stack = [(root, -1, iter(adj[root]))]
        while stack:
            u, pe, it = stack[-1]
            advanced = False
            for v, eid in it:
                if eid == pe:
                    continue
                if disc[v] == -1:
                    disc[v] = low[v] = timer
                    timer += 1
                    stack.append((v, eid, iter(adj[v])))
                    advanced = True
                    break
                low[u] = min(low[u], disc[v])
            if not advanced:
                stack.pop()
                if stack:
                    p = stack[-1][0]
                    low[p] = min(low[p], low[u])
                    if low[u] > disc[p]:
                        bridges.append(sorted([p, u]))
    return sorted(bridges)


if __name__ == "__main__":
    assert critical_connections(4, [[0, 1], [1, 2], [2, 0], [1, 3]]) == [[1, 3]]
    assert critical_connections(2, [[0, 1]]) == [[0, 1]]
    print("bridges:", critical_connections(6, [[0, 1], [1, 2], [2, 0], [1, 3], [3, 4], [4, 5], [5, 3]]))
