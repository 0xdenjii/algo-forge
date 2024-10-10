"""
Problem: LeetCode 876
Title: 876. Middle of the Linked List

Statement:
    Return the middle node of a singly linked list (second middle if even).

Approach:
    Fast advances two steps per slow step; slow ends at the middle.

Complexity: O(n) time, O(1) space.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional


@dataclass
class ListNode:
    val: int
    next: Optional[ListNode] = None


def middle_node(head: Optional[ListNode]) -> Optional[ListNode]:
    slow = fast = head
    while fast and fast.next:
        slow, fast = slow.next, fast.next.next
    return slow


def build(values: list[int]) -> Optional[ListNode]:
    head = None
    for v in reversed(values):
        head = ListNode(v, head)
    return head


if __name__ == "__main__":
    assert middle_node(build([1, 2, 3, 4, 5])).val == 3
    assert middle_node(build([1, 2, 3, 4, 5, 6])).val == 4
    assert middle_node(None) is None
    print("ok")
