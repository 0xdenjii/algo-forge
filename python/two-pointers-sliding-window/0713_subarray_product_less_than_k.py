"""
Problem: LeetCode 713
Title: 713. Subarray Product Less Than K

Statement:
    Count contiguous subarrays of positive integers whose product is strictly
    less than k.

Approach:
    Shrink while product >= k; every window ending at right with start in
    [left, right] is valid, adding right - left + 1.

Complexity: O(n) time, O(1) space.
"""

def num_subarray_product_less_than_k(nums: list[int], k: int) -> int:
    if k <= 1:
        return 0
    prod, left, count = 1, 0, 0
    for right, x in enumerate(nums):
        prod *= x
        while prod >= k:
            prod //= nums[left]
            left += 1
        count += right - left + 1
    return count


if __name__ == "__main__":
    assert num_subarray_product_less_than_k([10, 5, 2, 6], 100) == 8
    assert num_subarray_product_less_than_k([1, 2, 3], 0) == 0
    print("ok")
