# LeetCode 144 - Binary Tree Preorder Traversal
# Problem: Return the preorder (node, left, right) traversal of a binary tree.
# Approach: Stack-based (push right then left); also Morris traversal using
# temporary threads to the predecessor for O(1) extra space.
# Time: O(n)  Space: O(h) stack / O(1) Morris

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

def preorder_stack(root: Optional[TreeNode]) -> list[int]:
    res, stack = [], [root] if root else []
    while stack:
        node = stack.pop()
        res.append(node.val)
        if node.right:
            stack.append(node.right)
        if node.left:
            stack.append(node.left)
    return res


def preorder_morris(root: Optional[TreeNode]) -> list[int]:
    res, cur = [], root
    while cur:
        if not cur.left:
            res.append(cur.val)
            cur = cur.right
            continue
        pred = cur.left
        while pred.right and pred.right is not cur:
            pred = pred.right
        if pred.right is None:
            res.append(cur.val)
            pred.right = cur
            cur = cur.left
        else:
            pred.right = None
            cur = cur.right
    return res


if __name__ == "__main__":
    t = build([1, 2, 3, 4, 5, None, 6])
    assert preorder_stack(t) == preorder_morris(t) == [1, 2, 4, 5, 3, 6]
    assert to_list(t) == [1, 2, 3, 4, 5, None, 6]  # Morris restored the tree
    print("ok")
