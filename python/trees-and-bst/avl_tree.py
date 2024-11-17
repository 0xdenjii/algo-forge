# Classic - AVL Tree
# Problem: Implement a self-balancing BST supporting insert, delete and contains
# in guaranteed O(log n), keeping |height(left) - height(right)| <= 1 everywhere.
# Approach: Store height per node; after each insert/delete rebalance bottom-up
# with single (LL/RR) or double (LR/RL) rotations.
# Time: O(log n) per operation  Space: O(n)

from __future__ import annotations

import random
from typing import Iterator, Optional


class _Node:
    __slots__ = ("key", "left", "right", "height")

    def __init__(self, key: int):
        self.key = key
        self.left: Optional[_Node] = None
        self.right: Optional[_Node] = None
        self.height = 1


def _h(n: Optional[_Node]) -> int:
    return n.height if n else 0


def _update(n: _Node) -> None:
    n.height = 1 + max(_h(n.left), _h(n.right))


def _rotate_right(y: _Node) -> _Node:
    x = y.left
    y.left, x.right = x.right, y
    _update(y); _update(x)
    return x


def _rotate_left(x: _Node) -> _Node:
    y = x.right
    x.right, y.left = y.left, x
    _update(x); _update(y)
    return y


def _rebalance(n: _Node) -> _Node:
    _update(n)
    bal = _h(n.left) - _h(n.right)
    if bal > 1:
        if _h(n.left.left) < _h(n.left.right):
            n.left = _rotate_left(n.left)
        return _rotate_right(n)
    if bal < -1:
        if _h(n.right.right) < _h(n.right.left):
            n.right = _rotate_right(n.right)
        return _rotate_left(n)
    return n


class AVLTree:
    def __init__(self) -> None:
        self.root: Optional[_Node] = None
        self.size = 0

    def insert(self, key: int) -> None:
        def ins(n: Optional[_Node]) -> _Node:
            if not n:
                self.size += 1
                return _Node(key)
            if key < n.key:
                n.left = ins(n.left)
            elif key > n.key:
                n.right = ins(n.right)
            else:
                return n
            return _rebalance(n)

        self.root = ins(self.root)

    def delete(self, key: int) -> None:
        def dele(n: Optional[_Node], k: int) -> Optional[_Node]:
            if not n:
                return None
            if k < n.key:
                n.left = dele(n.left, k)
            elif k > n.key:
                n.right = dele(n.right, k)
            else:
                if not n.left or not n.right:
                    self.size -= 1
                    return n.left or n.right
                m = n.right
                while m.left:
                    m = m.left
                n.key = m.key
                n.right = dele(n.right, m.key)
            return _rebalance(n)

        self.root = dele(self.root, key)

    def __contains__(self, key: int) -> bool:
        n = self.root
        while n and n.key != key:
            n = n.left if key < n.key else n.right
        return n is not None

    def __iter__(self) -> Iterator[int]:
        stack, n = [], self.root
        while stack or n:
            while n:
                stack.append(n); n = n.left
            n = stack.pop()
            yield n.key
            n = n.right

    def __len__(self) -> int:
        return self.size

    def check(self) -> None:
        def go(n: Optional[_Node]) -> int:
            if not n:
                return 0
            l, r = go(n.left), go(n.right)
            assert abs(l - r) <= 1 and n.height == 1 + max(l, r)
            return n.height
        go(self.root)


if __name__ == "__main__":
    rng = random.Random(7)
    t, ref = AVLTree(), set()
    for _ in range(3000):
        k = rng.randrange(500)
        if rng.random() < 0.6:
            t.insert(k); ref.add(k)
        else:
            t.delete(k); ref.discard(k)
    t.check()
    assert list(t) == sorted(ref) and len(t) == len(ref)
    assert all((k in t) == (k in ref) for k in range(500))
    print("ok, height", t.root.height if t.root else 0, "for", len(t), "keys")
