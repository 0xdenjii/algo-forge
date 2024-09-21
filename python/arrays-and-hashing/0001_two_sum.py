# Problem: LeetCode 1 - Two Sum
#
# Given an integer array nums and a target, return indices of the two numbers
# that add up to target. Exactly one solution exists; an element may not be reused.
#
# Approach: One pass with a hash map value -> index; check complement before inserting.
# Complexity: O(n) time, O(n) space

def two_sum(nums: list[int], target: int) -> list[int]:
    seen: dict[int, int] = {}
    for i, x in enumerate(nums):
        if target - x in seen:
            return [seen[target - x], i]
        seen[x] = i
    raise ValueError("no solution")


if __name__ == "__main__":
    assert two_sum([2, 7, 11, 15], 9) == [0, 1]
    assert two_sum([3, 2, 4], 6) == [1, 2]
    assert two_sum([3, 3], 6) == [0, 1]
    print("ok")
