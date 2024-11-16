# LeetCode 449 - Serialize and Deserialize BST
# Problem: Encode a BST as compactly as possible and decode it back.
# Approach: Store preorder values only (no null markers); rebuild by consuming
# values within (lo, hi) bounds, which uniquely determines a BST.
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

class Codec:
    def serialize(self, root: Optional[TreeNode]) -> str:
        out: list[str] = []
        stack = [root] if root else []
        while stack:
            node = stack.pop()
            out.append(str(node.val))
            if node.right:
                stack.append(node.right)
            if node.left:
                stack.append(node.left)
        return " ".join(out)

    def deserialize(self, data: str) -> Optional[TreeNode]:
        vals = [int(x) for x in data.split()]
        i = 0

        def make(lo: float, hi: float) -> Optional[TreeNode]:
            nonlocal i
            if i == len(vals) or not lo < vals[i] < hi:
                return None
            node = TreeNode(vals[i]); i += 1
            node.left = make(lo, node.val)
            node.right = make(node.val, hi)
            return node

        return make(float("-inf"), float("inf"))


if __name__ == "__main__":
    c = Codec()
    for vals in ([2, 1, 3], [], [5, 3, 8, 1, 4, 7, 9, None, 2]):
        s = c.serialize(build(vals))
        assert to_list(c.deserialize(s)) == vals
    print("ok")
