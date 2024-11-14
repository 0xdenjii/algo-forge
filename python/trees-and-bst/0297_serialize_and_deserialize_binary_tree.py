# LeetCode 297 - Serialize and Deserialize Binary Tree
# Problem: Design an encoder/decoder converting a binary tree to a string and back.
# Approach: Preorder with '#' for null children, comma separated; decode by
# consuming tokens recursively from an iterator.
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

        def walk(node: Optional[TreeNode]) -> None:
            if not node:
                out.append("#")
                return
            out.append(str(node.val))
            walk(node.left)
            walk(node.right)

        walk(root)
        return ",".join(out)

    def deserialize(self, data: str) -> Optional[TreeNode]:
        tokens = iter(data.split(","))

        def make() -> Optional[TreeNode]:
            tok = next(tokens)
            if tok == "#":
                return None
            node = TreeNode(int(tok))
            node.left = make()
            node.right = make()
            return node

        return make()


if __name__ == "__main__":
    c = Codec()
    for vals in ([1, 2, 3, None, None, 4, 5], [], [-1, None, -2]):
        assert to_list(c.deserialize(c.serialize(build(vals)))) == vals
    print("ok")
