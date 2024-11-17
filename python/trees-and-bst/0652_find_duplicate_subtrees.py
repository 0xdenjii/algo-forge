# LeetCode 652 - Find Duplicate Subtrees
# Problem: Return one root node for every kind of subtree that appears at least twice.
# Approach: Assign each subtree an integer ID from (val, leftId, rightId); report
# a node when its ID's count reaches exactly 2.
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

from collections import Counter


def find_duplicate_subtrees(root: Optional[TreeNode]) -> list[TreeNode]:
    ids: dict[tuple, int] = {}
    count: Counter[int] = Counter()
    res: list[TreeNode] = []

    def ident(node: Optional[TreeNode]) -> int:
        if not node:
            return 0
        key = (node.val, ident(node.left), ident(node.right))
        i = ids.setdefault(key, len(ids) + 1)
        count[i] += 1
        if count[i] == 2:
            res.append(node)
        return i

    ident(root)
    return res


if __name__ == "__main__":
    dups = find_duplicate_subtrees(build([1, 2, 3, 4, None, 2, 4, None, None, 4]))
    assert sorted(to_list(d) for d in dups) == [[2, 4], [4]]
    print("ok")
