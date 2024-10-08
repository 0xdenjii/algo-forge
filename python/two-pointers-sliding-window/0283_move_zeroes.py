"""
Problem: LeetCode 283
Title: 283. Move Zeroes

Statement:
    Move all zeros to the end in place while keeping non-zero order.

Approach:
    Write pointer w; swap each non-zero into position w.

Complexity: O(n) time, O(1) space.
"""

def move_zeroes(nums: list[int]) -> None:
    w = 0
    for r in range(len(nums)):
        if nums[r]:
            nums[w], nums[r] = nums[r], nums[w]
            w += 1


if __name__ == "__main__":
    a = [0, 1, 0, 3, 12]
    move_zeroes(a)
    assert a == [1, 3, 12, 0, 0]
    print("ok")
