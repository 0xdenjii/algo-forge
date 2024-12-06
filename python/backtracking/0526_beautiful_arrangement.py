"""
LeetCode 526 - Beautiful Arrangement
Problem: Count permutations p of 1..n such that for each 1-indexed position i
either p[i] % i == 0 or i % p[i] == 0.
Approach: Fill positions from n down to 1 (the most constrained first) with a
bitmask of used values, memoized on (position, mask).
Time: O(n * 2^n)  Space: O(2^n)
"""
from functools import lru_cache


def count_arrangement(n: int) -> int:
    @lru_cache(maxsize=None)
    def dfs(pos: int, used: int) -> int:
        if pos == 0:
            return 1
        total = 0
        for v in range(1, n + 1):
            if not (used >> v) & 1 and (v % pos == 0 or pos % v == 0):
                total += dfs(pos - 1, used | (1 << v))
        return total

    return dfs(n, 0)


if __name__ == "__main__":
    assert count_arrangement(1) == 1
    assert count_arrangement(2) == 2
    assert count_arrangement(15) == 24679
    print(count_arrangement(10))
