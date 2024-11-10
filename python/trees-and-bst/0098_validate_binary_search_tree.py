# LeetCode 98 - Validate Binary Search Tree
# Problem: Determine if a binary tree is a valid BST (strict ordering).
# Approach: Iterative DFS carrying (low, high) exclusive bounds for each node.
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

def is_valid_bst(root: Optional[TreeNode]) -> bool:
    stack = [(root, float("-inf"), float("inf"))]
    while stack:
        node, lo, hi = stack.pop()
        if not node:
            continue
        if not lo < node.val < hi:
            return False
        stack.append((node.left, lo, node.val))
        stack.append((node.right, node.val, hi))
    return True


if __name__ == "__main__":
    assert is_valid_bst(build([2, 1, 3]))
    assert not is_valid_bst(build([5, 1, 4, None, None, 3, 6]))
    assert not is_valid_bst(build([5, 4, 6, None, None, 3, 7]))  # 3 violates root bound
    print("ok")
