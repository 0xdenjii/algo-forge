"""
LeetCode 78 - Subsets
Problem: Given an array of unique integers, return all possible subsets
(the power set) without duplicates.
Approach: Every node of the DFS tree is a subset; extend with later elements.
Time: O(n * 2^n)  Space: O(n)
"""
from typing import List


def subsets(nums: List[int]) -> List[List[int]]:
    out: List[List[int]] = []
    path: List[int] = []

    def dfs(start: int) -> None:
        out.append(path.copy())
        for i in range(start, len(nums)):
            path.append(nums[i])
            dfs(i + 1)
            path.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    assert subsets([1, 2, 3]) == [[], [1], [1, 2], [1, 2, 3], [1, 3], [2], [2, 3], [3]]
    assert subsets([0]) == [[], [0]]
    print("ok")
