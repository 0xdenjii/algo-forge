"""LeetCode 55 - Jump Game

nums[i] is the maximum jump length from index i. Starting at index 0,
return whether the last index is reachable.

Approach: greedy - track the farthest reachable index; fail if we ever stand
beyond it.
Time: O(n)  Space: O(1)
"""


def can_jump(nums: list[int]) -> bool:
    reach = 0
    for i, step in enumerate(nums):
        if i > reach:
            return False
        reach = max(reach, i + step)
    return True


if __name__ == "__main__":
    assert can_jump([2, 3, 1, 1, 4])
    assert not can_jump([3, 2, 1, 0, 4])
    assert can_jump([0])
    print("ok")
