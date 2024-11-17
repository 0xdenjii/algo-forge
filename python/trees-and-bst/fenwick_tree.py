# Classic - Fenwick Tree (Binary Indexed Tree)
# Problem: Maintain prefix sums under point additions; demo application counts
# array inversions (pairs i < j with a[i] > a[j]).
# Approach: tree[i] covers range (i - lowbit(i), i]; add/query walk by lowbit.
# Inversions: scan right-to-left, query how many smaller values were seen.
# Time: O(log n) per op, O(n log n) inversions  Space: O(n)

import random


class Fenwick:
    def __init__(self, n: int):
        self.n = n
        self.tree = [0] * (n + 1)

    def add(self, i: int, delta: int) -> None:
        """Add delta at 0-indexed position i."""
        i += 1
        while i <= self.n:
            self.tree[i] += delta
            i += i & -i

    def prefix(self, i: int) -> int:
        """Sum of positions [0, i)."""
        s = 0
        while i > 0:
            s += self.tree[i]
            i -= i & -i
        return s

    def range_sum(self, l: int, r: int) -> int:
        return self.prefix(r + 1) - self.prefix(l)


def count_inversions(a: list[int]) -> int:
    ranks = {v: i for i, v in enumerate(sorted(set(a)))}
    bit, inv = Fenwick(len(ranks)), 0
    for v in reversed(a):
        inv += bit.prefix(ranks[v])
        bit.add(ranks[v], 1)
    return inv


if __name__ == "__main__":
    assert count_inversions([2, 4, 1, 3, 5]) == 3
    rng = random.Random(5)
    a = [rng.randint(0, 20) for _ in range(60)]
    brute = sum(a[i] > a[j] for i in range(60) for j in range(i + 1, 60))
    assert count_inversions(a) == brute
    f = Fenwick(10)
    for i in range(10):
        f.add(i, i)
    assert f.range_sum(3, 6) == 3 + 4 + 5 + 6
    print("ok")
