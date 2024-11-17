# LeetCode 572 - Subtree of Another Tree
# Problem: Return True if `sub` is identical to some subtree of `root`.
# Approach: Canonical-ID hashing: map each (val, leftId, rightId) triple to a
# unique integer; sub is a subtree iff its ID appears among root's IDs.
# Time: O(n + m)  Space: O(n + m)

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

def is_subtree(root: Optional[TreeNode], sub: Optional[TreeNode]) -> bool:
    ids: dict[tuple, int] = {}
    seen: set[int] = set()

    def ident(node: Optional[TreeNode], collect: bool) -> int:
        if not node:
            return 0
        key = (node.val, ident(node.left, collect), ident(node.right, collect))
        i = ids.setdefault(key, len(ids) + 1)
        if collect:
            seen.add(i)
        return i

    ident(root, True)
    return ident(sub, False) in seen or sub is None


if __name__ == "__main__":
    assert is_subtree(build([3, 4, 5, 1, 2]), build([4, 1, 2]))
    assert not is_subtree(build([3, 4, 5, 1, 2, None, None, None, None, 0]), build([4, 1, 2]))
    print("ok")
