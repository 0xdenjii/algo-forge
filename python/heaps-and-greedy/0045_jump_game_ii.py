"""LeetCode 45 - Jump Game II

nums[i] is the maximum jump length from index i; the last index is always
reachable. Return the minimum number of jumps to reach it.

Approach: greedy BFS by levels - the current jump covers [.., end]; while
scanning it, record the farthest next reach, then jump when i hits end.
Time: O(n)  Space: O(1)
"""


def jump(nums: list[int]) -> int:
    jumps = end = farthest = 0
    for i in range(len(nums) - 1):
        farthest = max(farthest, i + nums[i])
        if i == end:
            jumps += 1
            end = farthest
    return jumps


if __name__ == "__main__":
    assert jump([2, 3, 1, 1, 4]) == 2
    assert jump([2, 3, 0, 1, 4]) == 2
    assert jump([0]) == 0
    print("ok")
