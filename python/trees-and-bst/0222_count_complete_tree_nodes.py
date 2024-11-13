# LeetCode 222 - Count Complete Tree Nodes
# Problem: Count nodes of a complete binary tree faster than O(n).
# Approach: Compare leftmost depths of left and right subtrees. Equal -> left
# subtree is perfect (2^h - 1 nodes); else right subtree is perfect one level shorter.
# Time: O(log^2 n)  Space: O(log n)

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

def count_nodes(root: Optional[TreeNode]) -> int:
    def left_depth(node: Optional[TreeNode]) -> int:
        d = 0
        while node:
            d += 1
            node = node.left
        return d

    if not root:
        return 0
    lh, rh = left_depth(root.left), left_depth(root.right)
    if lh == rh:
        return (1 << lh) + count_nodes(root.right)
    return (1 << rh) + count_nodes(root.left)


if __name__ == "__main__":
    for n in range(0, 40):
        assert count_nodes(build(list(range(1, n + 1)))) == n
    print("ok")
