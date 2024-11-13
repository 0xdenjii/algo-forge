# LeetCode 124 - Binary Tree Maximum Path Sum
# Problem: Find the maximum sum of any non-empty path (sequence of connected
# nodes, each used at most once); the path need not pass through the root.
# Approach: gain(node) = val + max(0, gain(left), gain(right)); update the global
# answer with val + max(0, gainL) + max(0, gainR).
# Time: O(n)  Space: O(h)

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

def max_path_sum(root: TreeNode) -> int:
    best = float("-inf")

    def gain(node: Optional[TreeNode]) -> int:
        nonlocal best
        if not node:
            return 0
        l, r = max(gain(node.left), 0), max(gain(node.right), 0)
        best = max(best, node.val + l + r)
        return node.val + max(l, r)

    gain(root)
    return int(best)


if __name__ == "__main__":
    assert max_path_sum(build([1, 2, 3])) == 6
    assert max_path_sum(build([-10, 9, 20, None, None, 15, 7])) == 42
    assert max_path_sum(build([-3])) == -3
    print("ok")
