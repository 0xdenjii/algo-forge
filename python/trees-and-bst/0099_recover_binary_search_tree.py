# LeetCode 99 - Recover Binary Search Tree
# Problem: Exactly two nodes of a BST were swapped by mistake; restore the BST
# without changing its structure.
# Approach: Morris inorder traversal; detect descents prev.val > cur.val. The first
# descent's prev and the last descent's cur are the swapped nodes.
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

def recover_tree(root: Optional[TreeNode]) -> None:
    first = second = prev = None
    cur = root

    def visit(node: TreeNode) -> None:
        nonlocal first, second, prev
        if prev and prev.val > node.val:
            if first is None:
                first = prev
            second = node
        prev = node

    while cur:
        if not cur.left:
            visit(cur)
            cur = cur.right
            continue
        pred = cur.left
        while pred.right and pred.right is not cur:
            pred = pred.right
        if pred.right is None:
            pred.right = cur
            cur = cur.left
        else:
            pred.right = None
            visit(cur)
            cur = cur.right
    if first and second:
        first.val, second.val = second.val, first.val


if __name__ == "__main__":
    t = build([1, 3, None, None, 2]); recover_tree(t)
    assert to_list(t) == [3, 1, None, None, 2]
    t = build([3, 1, 4, None, None, 2]); recover_tree(t)
    assert to_list(t) == [2, 1, 4, None, None, 3]
    print("ok")
