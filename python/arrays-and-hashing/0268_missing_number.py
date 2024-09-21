# Problem: LeetCode 268 - Missing Number
#
# nums contains n distinct numbers from [0, n]; return the one missing.
#
# Approach: XOR all indices 0..n with all values; pairs cancel, leaving the missing one.
# Complexity: O(n) time, O(1) space

from functools import reduce
from operator import xor


def missing_number(nums: list[int]) -> int:
    return reduce(xor, nums, 0) ^ reduce(xor, range(len(nums) + 1), 0)


if __name__ == "__main__":
    assert missing_number([3, 0, 1]) == 2
    assert missing_number([0, 1]) == 2
    assert missing_number([9, 6, 4, 2, 3, 5, 7, 0, 1]) == 8
    print("ok")
