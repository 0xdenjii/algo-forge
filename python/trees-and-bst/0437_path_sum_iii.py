# LeetCode 437 - Path Sum III
# Problem: Count paths (going downward, any start/end node) whose values sum to target.
# Approach: DFS with a hashmap of prefix-sum counts along the current root path;
# paths ending here = count[prefix - target]. Backtrack on return.
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

from collections import Counter


def path_sum(root: Optional[TreeNode], target: int) -> int:
    counts = Counter({0: 1})

    def dfs(node: Optional[TreeNode], prefix: int) -> int:
        if not node:
            return 0
        prefix += node.val
        found = counts[prefix - target]
        counts[prefix] += 1
        found += dfs(node.left, prefix) + dfs(node.right, prefix)
        counts[prefix] -= 1
        return found

    return dfs(root, 0)


if __name__ == "__main__":
    assert path_sum(build([10, 5, -3, 3, 2, None, 11, 3, -2, None, 1]), 8) == 3
    print("ok")
