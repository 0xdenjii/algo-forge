"""
LeetCode 491 - Non-decreasing Subsequences
Problem: Return all distinct non-decreasing subsequences of length >= 2.
Approach: DFS; each depth keeps a local 'seen' set so the same value is not
chosen twice at that level (the input is unsorted, so the sort trick fails).
Time: O(n * 2^n)  Space: O(n)
"""
from typing import List


def find_subsequences(nums: List[int]) -> List[List[int]]:
    out: List[List[int]] = []
    path: List[int] = []

    def dfs(start: int) -> None:
        if len(path) >= 2:
            out.append(path.copy())
        seen = set()
        for i in range(start, len(nums)):
            x = nums[i]
            if x in seen or (path and x < path[-1]):
                continue
            seen.add(x)
            path.append(x)
            dfs(i + 1)
            path.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    res = find_subsequences([4, 6, 7, 7])
    assert sorted(res) == sorted([[4, 6], [4, 6, 7], [4, 6, 7, 7], [4, 7], [4, 7, 7],
                                  [6, 7], [6, 7, 7], [7, 7]])
    assert find_subsequences([4, 4, 3, 2, 1]) == [[4, 4]]
    print("ok")
