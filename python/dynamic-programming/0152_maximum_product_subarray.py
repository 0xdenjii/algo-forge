# LeetCode 152 - Maximum Product Subarray
# Problem: Find the contiguous non-empty subarray with the largest product.
# Approach: Track both max and min product ending here, since a negative number
#   swaps them.
# Time: O(n)  Space: O(1)
from typing import Sequence


def max_product(nums: Sequence[int]) -> int:
    hi = lo = best = nums[0]
    for x in nums[1:]:
        cands = (x, hi * x, lo * x)
        hi, lo = max(cands), min(cands)
        best = max(best, hi)
    return best


if __name__ == "__main__":
    assert max_product([2, 3, -2, 4]) == 6
    assert max_product([-2, 0, -1]) == 0
    assert max_product([-2, 3, -4]) == 24
    print("ok")
