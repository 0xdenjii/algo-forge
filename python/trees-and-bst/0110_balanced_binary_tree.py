# LeetCode 110 - Balanced Binary Tree
# Problem: Determine whether every node's subtrees differ in height by at most one.
# Approach: Single post-order pass returning height, or -1 once imbalance is found.
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

def is_balanced(root: Optional[TreeNode]) -> bool:
    def height(node: Optional[TreeNode]) -> int:
        if not node:
            return 0
        lh = height(node.left)
        if lh < 0:
            return -1
        rh = height(node.right)
        if rh < 0 or abs(lh - rh) > 1:
            return -1
        return 1 + max(lh, rh)

    return height(root) >= 0


if __name__ == "__main__":
    assert is_balanced(build([3, 9, 20, None, None, 15, 7]))
    assert not is_balanced(build([1, 2, 2, 3, 3, None, None, 4, 4]))
    print("ok")
