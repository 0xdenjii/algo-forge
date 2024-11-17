# LeetCode 133 - Clone Graph
# Given a reference to a node in a connected undirected graph, return a deep copy.
# Each node has a value and a list of neighbors.
# Approach: DFS with a dict mapping original -> clone to handle cycles.
# Time: O(V + E)  Space: O(V)

from __future__ import annotations


class Node:
    def __init__(self, val: int = 0, neighbors: list[Node] | None = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


def clone_graph(node: Node | None) -> Node | None:
    if node is None:
        return None
    clones: dict[Node, Node] = {node: Node(node.val)}
    stack = [node]
    while stack:
        cur = stack.pop()
        for nb in cur.neighbors:
            if nb not in clones:
                clones[nb] = Node(nb.val)
                stack.append(nb)
            clones[cur].neighbors.append(clones[nb])
    return clones[node]


def to_adj(node: Node) -> dict[int, list[int]]:
    seen, out, stack = {node}, {}, [node]
    while stack:
        cur = stack.pop()
        out[cur.val] = sorted(n.val for n in cur.neighbors)
        for n in cur.neighbors:
            if n not in seen:
                seen.add(n)
                stack.append(n)
    return out


if __name__ == "__main__":
    a, b, c, d = Node(1), Node(2), Node(3), Node(4)
    a.neighbors, b.neighbors, c.neighbors, d.neighbors = [b, d], [a, c], [b, d], [a, c]
    copy = clone_graph(a)
    assert copy is not a and to_adj(copy) == to_adj(a)
    print("clone adjacency:", to_adj(copy))
