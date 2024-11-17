# LeetCode 987 - Vertical Order Traversal of a Binary Tree
# Problem: Group node values by column (left to right); within a column order by
# row, then by value for nodes sharing the same (row, col).
# Approach: DFS collecting (col, row, val) triples, sort once, then group by col.
# Time: O(n log n)  Space: O(n)

from __future__ import annotations

from collections import deque
from typing import Optional


class TreeNode:
    def __init__(self, val: int = 0, left: "Optional[TreeNode]" = None, right: "Optional[TreeNode]" = None):
        self.val = val
        self.left = left
        self.right = right


def build(values: list) -> Optional[TreeNode]:
    """Build a tree from LeetCode-style level-order list (None = missing)."""
    if not values or values[0] is None:
        return None
    root = TreeNode(values[0])
    queue, i = deque([root]), 1
    while queue and i < len(values):
        node = queue.popleft()
        if i < len(values) and values[i] is not None:
            node.left = TreeNode(values[i]); queue.append(node.left)
        i += 1
        if i < len(values) and values[i] is not None:
            node.right = TreeNode(values[i]); queue.append(node.right)
        i += 1
    return root


def to_list(root: Optional[TreeNode]) -> list:
    """Serialize to LeetCode-style level-order list, trailing Nones trimmed."""
    out, queue = [], deque([root])
    while queue:
        node = queue.popleft()
        if node:
            out.append(node.val); queue.append(node.left); queue.append(node.right)
        else:
            out.append(None)
    while out and out[-1] is None:
        out.pop()
    return out

from itertools import groupby


def vertical_traversal(root: Optional[TreeNode]) -> list[list[int]]:
    cells: list[tuple[int, int, int]] = []
    stack = [(root, 0, 0)] if root else []
    while stack:
        node, row, col = stack.pop()
        cells.append((col, row, node.val))
        if node.left:
            stack.append((node.left, row + 1, col - 1))
        if node.right:
            stack.append((node.right, row + 1, col + 1))
    cells.sort()
    return [[v for _, _, v in grp] for _, grp in groupby(cells, key=lambda c: c[0])]


if __name__ == "__main__":
    assert vertical_traversal(build([3, 9, 20, None, None, 15, 7])) == [[9], [3, 15], [20], [7]]
    assert vertical_traversal(build([1, 2, 3, 4, 6, 5, 7])) == [[4], [2], [1, 5, 6], [3], [7]]
    print("ok")
