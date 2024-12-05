"""
LeetCode 39 - Combination Sum
Problem: Given distinct positive candidates and a target, return all unique
combinations summing to target; each candidate may be reused unlimited times.
Approach: Sort; DFS with start index (reuse allowed, so recurse on the same i),
prune when the candidate exceeds the remaining sum.
Time: O(N^(T/min+1))  Space: O(T/min)
"""
from typing import List


def combination_sum(candidates: List[int], target: int) -> List[List[int]]:
    candidates = sorted(candidates)
    out: List[List[int]] = []
    path: List[int] = []

    def dfs(start: int, remain: int) -> None:
        if remain == 0:
            out.append(path.copy())
            return
        for i in range(start, len(candidates)):
            if candidates[i] > remain:
                break
            path.append(candidates[i])
            dfs(i, remain - candidates[i])
            path.pop()

    dfs(0, target)
    return out


if __name__ == "__main__":
    assert combination_sum([2, 3, 6, 7], 7) == [[2, 2, 3], [7]]
    assert combination_sum([2, 3, 5], 8) == [[2, 2, 2, 2], [2, 3, 3], [3, 5]]
    assert combination_sum([2], 1) == []
    print("ok")
