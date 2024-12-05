"""
LeetCode 40 - Combination Sum II
Problem: Given candidates (may contain duplicates) and a target, return all
unique combinations summing to target where each number is used at most once.
Approach: Sort; skip equal values at the same recursion depth to avoid
duplicate combinations; recurse with i + 1.
Time: O(2^n)  Space: O(n)
"""
from typing import List


def combination_sum2(candidates: List[int], target: int) -> List[List[int]]:
    nums = sorted(candidates)
    out: List[List[int]] = []
    path: List[int] = []

    def dfs(start: int, remain: int) -> None:
        if remain == 0:
            out.append(path.copy())
            return
        for i in range(start, len(nums)):
            if i > start and nums[i] == nums[i - 1]:
                continue
            if nums[i] > remain:
                break
            path.append(nums[i])
            dfs(i + 1, remain - nums[i])
            path.pop()

    dfs(0, target)
    return out


if __name__ == "__main__":
    assert combination_sum2([10, 1, 2, 7, 6, 1, 5], 8) == [[1, 1, 6], [1, 2, 5], [1, 7], [2, 6]]
    assert combination_sum2([2, 5, 2, 1, 2], 5) == [[1, 2, 2], [5]]
    print("ok")
