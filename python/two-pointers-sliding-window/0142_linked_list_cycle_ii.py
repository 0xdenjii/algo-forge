"""
Problem: LeetCode 142
Title: 142. Linked List Cycle II (with 141)

Statement:
    Detect whether a linked list has a cycle (141), and if so return the node
    where the cycle begins (142).

Approach:
    Fast moves 2, slow 1; they meet inside the cycle. Resetting one pointer to
    head and moving both by 1 meets at the entry.

Complexity: O(n) time, O(1) space.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional


@dataclass(eq=False)
class ListNode:
    val: int
    next: Optional[ListNode] = None


def detect_cycle(head: Optional[ListNode]) -> Optional[ListNode]:
    slow = fast = head
    while fast and fast.next:
        slow, fast = slow.next, fast.next.next
        if slow is fast:
            slow = head
            while slow is not fast:
                slow, fast = slow.next, fast.next
            return slow
    return None


def has_cycle(head: Optional[ListNode]) -> bool:
    return detect_cycle(head) is not None


if __name__ == "__main__":
    nodes = [ListNode(v) for v in (3, 2, 0, -4)]
    for a, b in zip(nodes, nodes[1:]):
        a.next = b
    assert not has_cycle(nodes[0])
    nodes[-1].next = nodes[1]
    assert detect_cycle(nodes[0]) is nodes[1]
    print("ok")
