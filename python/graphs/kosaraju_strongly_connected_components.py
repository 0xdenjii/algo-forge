# Kosaraju's Strongly Connected Components (classic algorithm)
# Given a directed graph, find its strongly connected components.
# Approach: DFS to get finish order; then DFS on the transposed graph in reverse finish order.
# Time: O(V + E)  Space: O(V + E)

def kosaraju_scc(n: int, adj: list[list[int]]) -> list[list[int]]:
    radj = [[] for _ in range(n)]
    for u in range(n):
        for v in adj[u]:
            radj[v].append(u)
    seen, order = [False] * n, []
    for s in range(n):
        if seen[s]:
            continue
        seen[s] = True
        stack = [(s, iter(adj[s]))]
        while stack:
            u, it = stack[-1]
            for v in it:
                if not seen[v]:
                    seen[v] = True
                    stack.append((v, iter(adj[v])))
                    break
            else:
                stack.pop()
                order.append(u)
    comp = [-1] * n
    sccs = []
    for s in reversed(order):
        if comp[s] != -1:
            continue
        comp[s] = len(sccs)
        members, stack = [], [s]
        while stack:
            u = stack.pop()
            members.append(u)
            for v in radj[u]:
                if comp[v] == -1:
                    comp[v] = len(sccs)
                    stack.append(v)
        sccs.append(sorted(members))
    return sccs


if __name__ == "__main__":
    adj = [[1], [2], [0, 3], [4], [5], [3], []]
    assert sorted(kosaraju_scc(7, adj)) == [[0, 1, 2], [3, 4, 5], [6]]
    print("sccs:", kosaraju_scc(7, adj))
