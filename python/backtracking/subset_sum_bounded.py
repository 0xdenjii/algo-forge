"""
Subset Sum (classic, backtracking with bounding)
Problem: Given positive integers and a target, find one subset that sums
exactly to target, or return None.
Approach: Sort descending; include/exclude recursion with two bounds:
skip inclusion when cur + nums[i] > target, and abandon the branch when
cur + (sum of all remaining) < target.
Time: O(2^n) worst  Space: O(n)
"""
from typing import List, Optional


def subset_sum(nums: List[int], target: int) -> Optional[List[int]]:
    nums = sorted(nums, reverse=True)
    suffix = [0] * (len(nums) + 1)
    for i in range(len(nums) - 1, -1, -1):
        suffix[i] = suffix[i + 1] + nums[i]
    chosen: List[int] = []

    def dfs(i: int, cur: int) -> bool:
        if cur == target:
            return True
        if i == len(nums) or cur + suffix[i] < target:
            return False
        if cur + nums[i] <= target:
            chosen.append(nums[i])
            if dfs(i + 1, cur + nums[i]):
                return True
            chosen.pop()
        return dfs(i + 1, cur)

    return chosen if dfs(0, 0) else None


if __name__ == "__main__":
    res = subset_sum([3, 34, 4, 12, 5, 2], 9)
    assert res is not None and sum(res) == 9
    assert subset_sum([3, 34, 4, 12, 5, 2], 30) is None
    assert subset_sum([], 0) == []
    print(res)
