# LeetCode 106 - Construct Binary Tree from Inorder and Postorder Traversal
# Problem: Given inorder and postorder traversals (unique values), rebuild the tree.
# Approach: Consume postorder from the end (root, then right subtree, then left);
# split inorder via a value->index map.
# Time: O(n)  Space: O(n)

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

def build_tree(inorder: list[int], postorder: list[int]) -> Optional[TreeNode]:
    index = {v: i for i, v in enumerate(inorder)}
    post = reversed(postorder)

    def make(lo: int, hi: int) -> Optional[TreeNode]:
        if lo > hi:
            return None
        val = next(post)
        node = TreeNode(val)
        mid = index[val]
        node.right = make(mid + 1, hi)
        node.left = make(lo, mid - 1)
        return node

    return make(0, len(inorder) - 1)


if __name__ == "__main__":
    assert to_list(build_tree([9, 3, 15, 20, 7], [9, 15, 7, 20, 3])) == [3, 9, 20, None, None, 15, 7]
    print("ok")
