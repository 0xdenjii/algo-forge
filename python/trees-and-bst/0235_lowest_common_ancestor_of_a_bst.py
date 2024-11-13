# LeetCode 235 - Lowest Common Ancestor of a Binary Search Tree
# Problem: Find the LCA of nodes p and q in a BST.
# Approach: Walk from root: if both values are smaller go left, both larger go
# right, otherwise the current node splits them and is the LCA.
# Time: O(h)  Space: O(1)

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

def lca_bst(root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:
    lo, hi = sorted((p.val, q.val))
    node = root
    while node:
        if hi < node.val:
            node = node.left
        elif lo > node.val:
            node = node.right
        else:
            return node
    raise ValueError("p and q must be in the tree")


if __name__ == "__main__":
    t = build([6, 2, 8, 0, 4, 7, 9, None, None, 3, 5])
    assert lca_bst(t, TreeNode(2), TreeNode(8)).val == 6
    assert lca_bst(t, TreeNode(2), TreeNode(4)).val == 2
    print("ok")
