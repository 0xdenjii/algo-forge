# Problem: LeetCode 260 - Single Number III
#
# Exactly two elements appear once; all others appear twice.
# Return the two singles in any order.
#
# Approach: XOR everything to get a ^ b; its lowest set bit splits numbers into two groups,
#   each containing exactly one single.
# Complexity: O(n) time, O(1) space

def single_number(nums: list[int]) -> list[int]:
    both = 0
    for x in nums:
        both ^= x
    low = both & -both
    a = 0
    for x in nums:
        if x & low:
            a ^= x
    return [a, both ^ a]


if __name__ == "__main__":
    assert sorted(single_number([1, 2, 1, 3, 2, 5])) == [3, 5]
    assert sorted(single_number([-1, 0])) == [-1, 0]
    print("ok")
