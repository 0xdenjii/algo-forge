# Classic - Segment Tree (point update, range query) / LeetCode 307 Range Sum Query - Mutable
# Problem: Support update(i, val) and sum_range(l, r) (inclusive) on an array.
# Approach: Bottom-up iterative segment tree of size 2n: leaves at [n, 2n),
# parents at i = (2i) + (2i+1). Queries climb from both ends.
# Time: O(log n) per op, O(n) build  Space: O(n)

import random


class SegmentTree:
    def __init__(self, nums: list[int]):
        self.n = n = len(nums)
        self.t = [0] * n + list(nums)
        for i in range(n - 1, 0, -1):
            self.t[i] = self.t[2 * i] + self.t[2 * i + 1]

    def update(self, i: int, val: int) -> None:
        i += self.n
        self.t[i] = val
        while i > 1:
            i //= 2
            self.t[i] = self.t[2 * i] + self.t[2 * i + 1]

    def sum_range(self, l: int, r: int) -> int:
        res, l, r = 0, l + self.n, r + self.n + 1
        while l < r:
            if l & 1:
                res += self.t[l]; l += 1
            if r & 1:
                r -= 1; res += self.t[r]
            l //= 2; r //= 2
        return res


if __name__ == "__main__":
    st = SegmentTree([1, 3, 5])
    assert st.sum_range(0, 2) == 9
    st.update(1, 2)
    assert st.sum_range(0, 2) == 8
    rng = random.Random(3)
    arr = [rng.randint(-50, 50) for _ in range(37)]
    st = SegmentTree(arr)
    for _ in range(500):
        if rng.random() < 0.5:
            i, v = rng.randrange(37), rng.randint(-50, 50)
            arr[i] = v; st.update(i, v)
        else:
            l = rng.randrange(37); r = rng.randrange(l, 37)
            assert st.sum_range(l, r) == sum(arr[l:r + 1])
    print("ok")
