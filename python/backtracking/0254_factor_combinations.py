"""
LeetCode 254 - Factor Combinations
Problem: Return all ways to write n as a product of factors in [2, n-1],
each list in non-decreasing order.
Approach: DFS: for each factor f >= start with f*f <= num, record
path + [f, num // f] and recurse into num // f with start f.
Time: about O(number of factorizations * sqrt(n))  Space: O(log n)
"""
from typing import List


def get_factors(n: int) -> List[List[int]]:
    out: List[List[int]] = []

    def dfs(num: int, start: int, path: List[int]) -> None:
        f = start
        while f * f <= num:
            if num % f == 0:
                out.append(path + [f, num // f])
                dfs(num // f, f, path + [f])
            f += 1

    dfs(n, 2, [])
    return out


if __name__ == "__main__":
    assert get_factors(1) == []
    assert get_factors(37) == []
    assert get_factors(12) == [[2, 6], [2, 2, 3], [3, 4]]
    print(get_factors(32))
