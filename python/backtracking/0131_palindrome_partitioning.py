"""
LeetCode 131 - Palindrome Partitioning
Problem: Partition string s so every substring is a palindrome; return all
such partitions.
Approach: Precompute pal[i][j] with DP, then DFS over cut positions.
Time: O(n * 2^n)  Space: O(n^2)
"""
from typing import List


def partition(s: str) -> List[List[str]]:
    n = len(s)
    pal = [[False] * n for _ in range(n)]
    for i in range(n - 1, -1, -1):
        for j in range(i, n):
            pal[i][j] = s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1])

    out: List[List[str]] = []
    path: List[str] = []

    def dfs(i: int) -> None:
        if i == n:
            out.append(path.copy())
            return
        for j in range(i, n):
            if pal[i][j]:
                path.append(s[i:j + 1])
                dfs(j + 1)
                path.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    assert partition("aab") == [["a", "a", "b"], ["aa", "b"]]
    assert partition("a") == [["a"]]
    print(partition("racecar"))
