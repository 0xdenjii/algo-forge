# Problem: LeetCode 238 - Product of Array Except Self
#
# Return an array where answer[i] is the product of all nums except nums[i],
# without using division.
#
# Approach: Prefix products left-to-right into the output, then multiply by a running suffix.
# Complexity: O(n) time, O(1) extra space (output excluded)

def product_except_self(nums: list[int]) -> list[int]:
    n = len(nums)
    out = [1] * n
    prefix = 1
    for i in range(n):
        out[i] = prefix
        prefix *= nums[i]
    suffix = 1
    for i in range(n - 1, -1, -1):
        out[i] *= suffix
        suffix *= nums[i]
    return out


if __name__ == "__main__":
    assert product_except_self([1, 2, 3, 4]) == [24, 12, 8, 6]
    assert product_except_self([-1, 1, 0, -3, 3]) == [0, 0, 9, 0, 0]
    print("ok")
