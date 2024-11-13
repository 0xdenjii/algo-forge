# LeetCode 116 - Populating Next Right Pointers in Each Node
# Problem: In a perfect binary tree, set each node's `next` to its right neighbour
# on the same level (None for the rightmost).
# Approach: Use already-built `next` links of level L to wire level L+1:
# left.next = right, right.next = node.next.left.
# Time: O(n)  Space: O(1)

from typing import Optional


class Node:
    def __init__(self, val: int, left: "Optional[Node]" = None, right: "Optional[Node]" = None):
        self.val, self.left, self.right = val, left, right
        self.next: Optional[Node] = None


def connect(root: Optional[Node]) -> Optional[Node]:
    leftmost = root
    while leftmost and leftmost.left:
        node = leftmost
        while node:
            node.left.next = node.right
            if node.next:
                node.right.next = node.next.left
            node = node.next
        leftmost = leftmost.left
    return root


if __name__ == "__main__":
    r = Node(1, Node(2, Node(4), Node(5)), Node(3, Node(6), Node(7)))
    connect(r)
    levels, start = [], r
    while start:
        row, n = [], start
        while n:
            row.append(n.val)
            n = n.next
        levels.append(row)
        start = start.left
    assert levels == [[1], [2, 3], [4, 5, 6, 7]]
    print("ok")
