# LeetCode 173 - Binary Search Tree Iterator
# Problem: Implement an iterator over a BST in ascending order with next() and
# hasNext() running in amortized O(1) time and O(h) memory.
# Approach: Controlled inorder traversal: keep the left spine of the unvisited part on a stack.
# Time: amortized O(1) per call  Space: O(h)

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

class BSTIterator:
    def __init__(self, root: Optional[TreeNode]):
        self._stack: list[TreeNode] = []
        self._push_left(root)

    def _push_left(self, node: Optional[TreeNode]) -> None:
        while node:
            self._stack.append(node)
            node = node.left

    def next(self) -> int:
        node = self._stack.pop()
        self._push_left(node.right)
        return node.val

    def has_next(self) -> bool:
        return bool(self._stack)

    def __iter__(self):
        while self.has_next():
            yield self.next()


if __name__ == "__main__":
    assert list(BSTIterator(build([7, 3, 15, None, None, 9, 20]))) == [3, 7, 9, 15, 20]
    print("ok")
