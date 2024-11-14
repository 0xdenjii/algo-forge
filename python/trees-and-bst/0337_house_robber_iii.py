# LeetCode 337 - House Robber III
# Problem: Houses form a binary tree; robbing two directly-linked houses alerts
# police. Return the maximum amount you can rob.
# Approach: Post-order DP returning (best if node robbed, best if skipped).
# Time: O(n)  Space: O(h)

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

def rob(root: Optional[TreeNode]) -> int:
    def dp(node: Optional[TreeNode]) -> tuple[int, int]:
        if not node:
            return 0, 0
        lr, ls = dp(node.left)
        rr, rs = dp(node.right)
        return node.val + ls + rs, max(lr, ls) + max(rr, rs)

    return max(dp(root))


if __name__ == "__main__":
    assert rob(build([3, 2, 3, None, 3, None, 1])) == 7
    assert rob(build([3, 4, 5, 1, 3, None, 1])) == 9
    print("ok")
