# LeetCode 103 - Binary Tree Zigzag Level Order Traversal
# Problem: Level order traversal, alternating left-to-right and right-to-left.
# Approach: Standard BFS; fill each level into a deque from the side
# dictated by the current direction.
# Time: O(n)  Space: O(w)

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

def zigzag_level_order(root: Optional[TreeNode]) -> list[list[int]]:
    if not root:
        return []
    res, q, ltr = [], deque([root]), True
    while q:
        level: deque[int] = deque()
        for _ in range(len(q)):
            node = q.popleft()
            (level.append if ltr else level.appendleft)(node.val)
            if node.left:
                q.append(node.left)
            if node.right:
                q.append(node.right)
        res.append(list(level))
        ltr = not ltr
    return res


if __name__ == "__main__":
    assert zigzag_level_order(build([3, 9, 20, None, None, 15, 7])) == [[3], [20, 9], [15, 7]]
    print("ok")
