# Problem: LeetCode 136 - Single Number
#
# Every element appears twice except one; find it in O(n) time, O(1) space.
#
# Approach: XOR everything; duplicates cancel out.
# Complexity: O(n) time, O(1) space

from functools import reduce
from operator import xor


def single_number(nums: list[int]) -> int:
    return reduce(xor, nums, 0)


if __name__ == "__main__":
    assert single_number([2, 2, 1]) == 1
    assert single_number([4, 1, 2, 1, 2]) == 4
    print("ok")
