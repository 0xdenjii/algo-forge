# Classic - Treap (randomized BST) with split/merge
# Problem: Maintain a dynamic ordered multiset supporting insert, erase, k-th
# smallest and rank (count of keys < x) in expected O(log n).
# Approach: BST on keys + heap on random priorities. All operations via split(t, k)
# into (< k, >= k) and merge; nodes store subtree sizes for order statistics.
# Time: expected O(log n) per op  Space: O(n)

from __future__ import annotations

import random
from typing import Optional

_rng = random.Random(2024)


class _Node:
    __slots__ = ("key", "prio", "left", "right", "size")

    def __init__(self, key: int):
        self.key, self.prio = key, _rng.random()
        self.left: Optional[_Node] = None
        self.right: Optional[_Node] = None
        self.size = 1


def _sz(t: Optional[_Node]) -> int:
    return t.size if t else 0


def _pull(t: _Node) -> _Node:
    t.size = 1 + _sz(t.left) + _sz(t.right)
    return t


def split(t: Optional[_Node], key: int) -> tuple[Optional[_Node], Optional[_Node]]:
    """Split into (keys < key, keys >= key)."""
    if not t:
        return None, None
    if t.key < key:
        l, r = split(t.right, key)
        t.right = l
        return _pull(t), r
    l, r = split(t.left, key)
    t.left = r
    return l, _pull(t)


def merge(a: Optional[_Node], b: Optional[_Node]) -> Optional[_Node]:
    """Merge where every key in a <= every key in b."""
    if not a or not b:
        return a or b
    if a.prio > b.prio:
        a.right = merge(a.right, b)
        return _pull(a)
    b.left = merge(a, b.left)
    return _pull(b)


class Treap:
    def __init__(self) -> None:
        self.root: Optional[_Node] = None

    def __len__(self) -> int:
        return _sz(self.root)

    def insert(self, key: int) -> None:
        l, r = split(self.root, key)
        self.root = merge(merge(l, _Node(key)), r)

    def erase(self, key: int) -> bool:
        """Remove one occurrence of key; return whether it existed."""
        l, r = split(self.root, key)
        mid, r = split(r, key + 1)
        found = mid is not None
        if mid:
            mid = merge(mid.left, mid.right)
        self.root = merge(merge(l, mid), r)
        return found

    def rank(self, key: int) -> int:
        n, res = self.root, 0
        while n:
            if n.key < key:
                res += _sz(n.left) + 1
                n = n.right
            else:
                n = n.left
        return res

    def kth(self, k: int) -> int:
        """0-indexed k-th smallest."""
        if not 0 <= k < len(self):
            raise IndexError(k)
        n = self.root
        while True:
            ls = _sz(n.left)
            if k < ls:
                n = n.left
            elif k == ls:
                return n.key
            else:
                k -= ls + 1
                n = n.right


if __name__ == "__main__":
    rng = random.Random(1)
    t, ref = Treap(), []
    for _ in range(2000):
        x = rng.randrange(100)
        if rng.random() < 0.65:
            t.insert(x); ref.append(x)
        else:
            assert t.erase(x) == (x in ref)
            if x in ref:
                ref.remove(x)
    ref.sort()
    assert len(t) == len(ref)
    assert [t.kth(i) for i in range(len(ref))] == ref
    assert all(t.rank(x) == sum(v < x for v in ref) for x in range(101))
    print("ok")
