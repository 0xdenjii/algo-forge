"""
LeetCode 47 - Permutations II
Problem: Given a collection of numbers that may contain duplicates, return all
unique permutations.
Approach: Sort and keep a used[] array; skip nums[i] if it equals nums[i-1]
and nums[i-1] is not currently used (duplicates are then taken in order).
Time: O(n * n!)  Space: O(n)
"""
from typing import List


def permute_unique(nums: List[int]) -> List[List[int]]:
    nums = sorted(nums)
    used = [False] * len(nums)
    out: List[List[int]] = []
    path: List[int] = []

    def dfs() -> None:
        if len(path) == len(nums):
            out.append(path.copy())
            return
        for i, x in enumerate(nums):
            if used[i] or (i > 0 and x == nums[i - 1] and not used[i - 1]):
                continue
            used[i] = True
            path.append(x)
            dfs()
            path.pop()
            used[i] = False

    dfs()
    return out


if __name__ == "__main__":
    assert permute_unique([1, 1, 2]) == [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
    assert len(permute_unique([1, 2, 3])) == 6
    print("ok")
