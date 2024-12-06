"""
LeetCode 698 - Partition to K Equal Sum Subsets
Problem: Decide whether nums can be divided into k non-empty subsets with
equal sums.
Approach: Fill buckets one at a time over a bitmask of used elements,
memoizing masks; the current bucket fill is carried as (sum % target). If the
first item placed into an empty bucket fails, no other choice can succeed.
Time: O(n * 2^n)  Space: O(2^n)
"""
from functools import lru_cache
from typing import List


def can_partition_k_subsets(nums: List[int], k: int) -> bool:
    total = sum(nums)
    if total % k:
        return False
    target = total // k
    nums = sorted(nums, reverse=True)
    if nums[0] > target:
        return False
    n = len(nums)
    full = (1 << n) - 1

    @lru_cache(maxsize=None)
    def dfs(mask: int, fill: int) -> bool:
        if mask == full:
            return True
        for i in range(n):
            if (mask >> i) & 1 or fill + nums[i] > target:
                continue
            if dfs(mask | (1 << i), (fill + nums[i]) % target):
                return True
            if fill == 0:
                break
        return False

    return dfs(0, 0)


if __name__ == "__main__":
    assert can_partition_k_subsets([4, 3, 2, 3, 5, 2, 1], 4)
    assert not can_partition_k_subsets([1, 2, 3, 4], 3)
    assert not can_partition_k_subsets([2, 2, 2, 2, 3, 4, 5], 4)
    print("ok")
