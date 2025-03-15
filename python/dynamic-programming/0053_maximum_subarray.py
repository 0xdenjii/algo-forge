# LeetCode 53 - Maximum Subarray
# Problem: Find the contiguous non-empty subarray with the largest sum.
# Approach: Kadane: best ending here = max(x, best ending at previous + x).
# Time: O(n)  Space: O(1)
from typing import Sequence


def max_sub_array(nums: Sequence[int]) -> int:
    cur = best = nums[0]
    for x in nums[1:]:
        cur = max(x, cur + x)
        best = max(best, cur)
    return best


if __name__ == "__main__":
    assert max_sub_array([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6
    assert max_sub_array([-3, -1, -2]) == -1
    print("ok")
