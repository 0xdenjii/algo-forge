# LeetCode 538 - Convert BST to Greater Tree
# Problem: Replace each key with itself plus the sum of all keys greater than it.
# Approach: Reverse inorder (right, node, left) carrying a running total.
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

def convert_bst(root: Optional[TreeNode]) -> Optional[TreeNode]:
    total, stack, node = 0, [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.right
        node = stack.pop()
        total += node.val
        node.val = total
        node = node.left
    return root


if __name__ == "__main__":
    t = convert_bst(build([4, 1, 6, 0, 2, 5, 7, None, None, None, 3, None, None, None, 8]))
    assert to_list(t) == [30, 36, 21, 36, 35, 26, 15, None, None, None, 33, None, None, None, 8]
    print("ok")
