# Problem: LeetCode 137 - Single Number II
#
# Every element appears three times except one, which appears once.
# Find it in O(n) time and O(1) space.
#
# Approach: Two-bitmask state machine (ones, twos) counting each bit modulo 3.
# Complexity: O(n) time, O(1) space

def single_number(nums: list[int]) -> int:
    ones = twos = 0
    for x in nums:
        ones = (ones ^ x) & ~twos
        twos = (twos ^ x) & ~ones
    return ones


if __name__ == "__main__":
    assert single_number([2, 2, 3, 2]) == 3
    assert single_number([0, 1, 0, 1, 0, 1, 99]) == 99
    assert single_number([-2, -2, 1, 1, 4, 1, 4, 4, -4, -2]) == -4
    print("ok")
