# LeetCode 114 - Flatten Binary Tree to Linked List
# Problem: Flatten the tree in place into a "linked list" in preorder using right
# pointers, with all left pointers set to None.
# Approach: Morris-like: for each node with a left child, attach its right subtree
# to the rightmost node of the left subtree, then move left subtree to the right.
# Time: O(n)  Space: O(1)

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

def flatten(root: Optional[TreeNode]) -> None:
    cur = root
    while cur:
        if cur.left:
            tail = cur.left
            while tail.right:
                tail = tail.right
            tail.right = cur.right
            cur.right, cur.left = cur.left, None
        cur = cur.right


if __name__ == "__main__":
    t = build([1, 2, 5, 3, 4, None, 6])
    flatten(t)
    vals = []
    while t:
        assert t.left is None
        vals.append(t.val)
        t = t.right
    assert vals == [1, 2, 3, 4, 5, 6]
    print("ok")
