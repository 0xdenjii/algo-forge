# LeetCode 108 - Convert Sorted Array to Binary Search Tree
# Problem: Build a height-balanced BST from a strictly increasing array.
# Approach: Choose the middle element as root; recurse on halves by index.
# Time: O(n)  Space: O(log n)

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

def sorted_array_to_bst(nums: list[int]) -> Optional[TreeNode]:
    def make(lo: int, hi: int) -> Optional[TreeNode]:
        if lo > hi:
            return None
        mid = (lo + hi) // 2
        return TreeNode(nums[mid], make(lo, mid - 1), make(mid + 1, hi))

    return make(0, len(nums) - 1)


def _inorder(n):
    return _inorder(n.left) + [n.val] + _inorder(n.right) if n else []


def _height(n):
    return 1 + max(_height(n.left), _height(n.right)) if n else 0


if __name__ == "__main__":
    nums = list(range(-10, 11))
    root = sorted_array_to_bst(nums)
    assert _inorder(root) == nums
    assert _height(root) == 5  # ceil(log2(22))
    print("ok")
