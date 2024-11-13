# LeetCode 112 - Path Sum
# Problem: Return True if some root-to-leaf path sums to targetSum.
# Approach: Subtract node values going down; check remainder at leaves.
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

def has_path_sum(root: Optional[TreeNode], target: int) -> bool:
    if not root:
        return False
    target -= root.val
    if not root.left and not root.right:
        return target == 0
    return has_path_sum(root.left, target) or has_path_sum(root.right, target)


if __name__ == "__main__":
    t = build([5, 4, 8, 11, None, 13, 4, 7, 2, None, None, None, 1])
    assert has_path_sum(t, 22)
    assert not has_path_sum(t, 100)
    print("ok")
