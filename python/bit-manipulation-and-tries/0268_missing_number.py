# Problem: LeetCode 268 - Missing Number
#
# nums contains n distinct numbers from [0, n]. Return the one missing.
#
# Approach: XOR all indices 0..n with all values; everything cancels except the missing one.
# Complexity: O(n) time, O(1) space

def missing_number(nums: list[int]) -> int:
    res = len(nums)
    for i, x in enumerate(nums):
        res ^= i ^ x
    return res


if __name__ == "__main__":
    assert missing_number([3, 0, 1]) == 2
    assert missing_number([9, 6, 4, 2, 3, 5, 7, 0, 1]) == 8
    print("ok")
