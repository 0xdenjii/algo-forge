"""
LeetCode 473 - Matchsticks to Square
Problem: Given matchstick lengths, decide whether all of them can form a
square (each stick used exactly once, none broken).
Approach: Sort descending and assign each stick to one of 4 sides, pruning
side overflow and skipping sides with an equal current length (symmetry).
Time: O(4^n) worst, heavily pruned  Space: O(n)
"""
from typing import List


def makesquare(sticks: List[int]) -> bool:
    total = sum(sticks)
    if len(sticks) < 4 or total % 4:
        return False
    side = total // 4
    sticks = sorted(sticks, reverse=True)
    if sticks[0] > side:
        return False
    sides = [0] * 4

    def dfs(i: int) -> bool:
        if i == len(sticks):
            return True
        tried = set()
        for k in range(4):
            if sides[k] + sticks[i] <= side and sides[k] not in tried:
                tried.add(sides[k])
                sides[k] += sticks[i]
                if dfs(i + 1):
                    return True
                sides[k] -= sticks[i]
        return False

    return dfs(0)


if __name__ == "__main__":
    assert makesquare([1, 1, 2, 2, 2])
    assert not makesquare([3, 3, 3, 3, 4])
    assert makesquare([5] * 4 + [1] * 8)
    print("ok")
