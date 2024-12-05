"""
LeetCode 90 - Subsets II
Problem: Given an integer array that may contain duplicates, return all
possible subsets without duplicate subsets.
Approach: Sort, then at each depth skip a value equal to its predecessor
in the same loop.
Time: O(n * 2^n)  Space: O(n)
"""
from typing import List


def subsets_with_dup(nums: List[int]) -> List[List[int]]:
    nums = sorted(nums)
    out: List[List[int]] = []
    path: List[int] = []

    def dfs(start: int) -> None:
        out.append(path.copy())
        for i in range(start, len(nums)):
            if i > start and nums[i] == nums[i - 1]:
                continue
            path.append(nums[i])
            dfs(i + 1)
            path.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    assert subsets_with_dup([1, 2, 2]) == [[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]
    print("ok")
