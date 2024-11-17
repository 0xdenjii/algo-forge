# LeetCode 968 - Binary Tree Cameras
# Problem: Cameras at a node monitor its parent, itself and its children. Return
# the minimum number of cameras needed to monitor every node.
# Approach: Greedy post-order with states 0=not covered, 1=has camera,
# 2=covered without camera. Place a camera whenever a child is uncovered.
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

def min_camera_cover(root: Optional[TreeNode]) -> int:
    cams = 0

    def state(node: Optional[TreeNode]) -> int:
        nonlocal cams
        if not node:
            return 2
        l, r = state(node.left), state(node.right)
        if l == 0 or r == 0:
            cams += 1
            return 1
        return 2 if l == 1 or r == 1 else 0

    if state(root) == 0:
        cams += 1
    return cams


if __name__ == "__main__":
    assert min_camera_cover(build([0, 0, None, 0, 0])) == 1
    assert min_camera_cover(build([0, 0, None, 0, None, 0, None, None, 0])) == 2
    assert min_camera_cover(build([0])) == 1
    print("ok")
