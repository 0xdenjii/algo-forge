# LeetCode 337 - House Robber III
# Problem: Houses form a binary tree; robbing two directly-linked houses alerts
#   the police. Return the maximum amount robbable.
# Approach: Tree DP returning (best if node robbed, best if node skipped).
# Time: O(n)  Space: O(h) recursion
from __future__ import annotations

from dataclasses import dataclass


@dataclass
class TreeNode:
    val: int
    left: TreeNode | None = None
    right: TreeNode | None = None


def rob(root: TreeNode | None) -> int:
    def go(node: TreeNode | None) -> tuple[int, int]:
        if node is None:
            return 0, 0
        lr, ls = go(node.left)
        rr, rs = go(node.right)
        return node.val + ls + rs, max(lr, ls) + max(rr, rs)

    return max(go(root))


if __name__ == "__main__":
    t = TreeNode(3, TreeNode(2, None, TreeNode(3)), TreeNode(3, None, TreeNode(1)))
    assert rob(t) == 7
    t2 = TreeNode(3, TreeNode(4, TreeNode(1), TreeNode(3)), TreeNode(5, None, TreeNode(1)))
    assert rob(t2) == 9
    assert rob(None) == 0
    print("ok")
