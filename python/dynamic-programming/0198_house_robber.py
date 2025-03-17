# LeetCode 198 - House Robber
# Problem: Houses in a row hold money; you cannot rob two adjacent houses.
#   Return the maximum amount you can rob.
# Approach: best(i) = max(best(i-1), best(i-2) + nums[i]), rolling two variables.
# Time: O(n)  Space: O(1)
from typing import Sequence


def rob(nums: Sequence[int]) -> int:
    prev, cur = 0, 0
    for x in nums:
        prev, cur = cur, max(cur, prev + x)
    return cur


if __name__ == "__main__":
    assert rob([1, 2, 3, 1]) == 4
    assert rob([2, 7, 9, 3, 1]) == 12
    assert rob([]) == 0
    print("ok")
