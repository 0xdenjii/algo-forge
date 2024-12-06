"""
LeetCode 216 - Combination Sum III
Problem: Find all combinations of k distinct numbers from 1..9 that sum to n.
Approach: DFS over increasing digits with sum and size pruning.
Time: O(C(9, k) * k)  Space: O(k)
"""
from typing import List


def combination_sum3(k: int, n: int) -> List[List[int]]:
    out: List[List[int]] = []
    path: List[int] = []

    def dfs(start: int, remain: int) -> None:
        if len(path) == k:
            if remain == 0:
                out.append(path.copy())
            return
        for d in range(start, 10):
            if d > remain:
                break
            path.append(d)
            dfs(d + 1, remain - d)
            path.pop()

    dfs(1, n)
    return out


if __name__ == "__main__":
    assert combination_sum3(3, 7) == [[1, 2, 4]]
    assert combination_sum3(3, 9) == [[1, 2, 6], [1, 3, 5], [2, 3, 4]]
    assert combination_sum3(4, 1) == []
    print("ok")
