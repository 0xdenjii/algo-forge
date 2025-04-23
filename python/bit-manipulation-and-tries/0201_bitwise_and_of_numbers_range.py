# Problem: LeetCode 201 - Bitwise AND of Numbers Range
#
# Return the bitwise AND of all numbers in [left, right].
#
# Approach: Result is the common binary prefix: strip lowest bits of right until right <= left.
# Complexity: O(log n) time, O(1) space

def range_bitwise_and(left: int, right: int) -> int:
    while left < right:
        right &= right - 1
    return right


if __name__ == "__main__":
    assert range_bitwise_and(5, 7) == 4
    assert range_bitwise_and(0, 0) == 0
    assert range_bitwise_and(1, 2147483647) == 0
    print("ok")
