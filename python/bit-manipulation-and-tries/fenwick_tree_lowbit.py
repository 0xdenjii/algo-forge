# Problem: Fenwick Tree (Binary Indexed Tree)
#
# Support point updates and prefix-sum queries on an array, plus range sums.
# A classic structure built on the lowbit trick i & -i.
#
# Approach: Node i covers (i - lowbit(i), i]; update climbs by +lowbit, query descends by -lowbit.
# Complexity: O(log n) per op, O(n) space

from __future__ import annotations


class FenwickTree:
    def __init__(self, n: int) -> None:
        self.n = n
        self.tree = [0] * (n + 1)

    @classmethod
    def from_list(cls, values: list[int]) -> FenwickTree:
        ft = cls(len(values))
        for i, v in enumerate(values, 1):
            ft.tree[i] += v
            j = i + (i & -i)
            if j <= ft.n:
                ft.tree[j] += ft.tree[i]
        return ft

    def add(self, i: int, delta: int) -> None:
        i += 1
        while i <= self.n:
            self.tree[i] += delta
            i += i & -i

    def prefix_sum(self, i: int) -> int:
        # sum of values[0:i]
        s = 0
        while i > 0:
            s += self.tree[i]
            i -= i & -i
        return s

    def range_sum(self, lo: int, hi: int) -> int:
        # inclusive range [lo, hi]
        return self.prefix_sum(hi + 1) - self.prefix_sum(lo)


if __name__ == "__main__":
    vals = [5, 2, 9, -3, 7, 1]
    ft = FenwickTree.from_list(vals)
    assert ft.range_sum(1, 3) == 8
    ft.add(3, 10)
    vals[3] += 10
    for lo in range(len(vals)):
        for hi in range(lo, len(vals)):
            assert ft.range_sum(lo, hi) == sum(vals[lo:hi + 1])
    print("ok")
