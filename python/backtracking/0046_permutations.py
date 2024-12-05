"""
LeetCode 46 - Permutations
Problem: Given an array of distinct integers, return all possible
permutations in any order.
Approach: In-place swapping: fix position i by swapping each later element in.
Time: O(n * n!)  Space: O(n)
"""
from typing import List


def permute(nums: List[int]) -> List[List[int]]:
    arr = list(nums)
    out: List[List[int]] = []

    def dfs(i: int) -> None:
        if i == len(arr):
            out.append(arr.copy())
            return
        for j in range(i, len(arr)):
            arr[i], arr[j] = arr[j], arr[i]
            dfs(i + 1)
            arr[i], arr[j] = arr[j], arr[i]

    dfs(0)
    return out


if __name__ == "__main__":
    res = permute([1, 2, 3])
    assert sorted(res) == [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
    assert permute([1]) == [[1]]
    print(res)
