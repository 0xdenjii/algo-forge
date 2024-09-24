# Problem: LeetCode 560 - Subarray Sum Equals K
#
# Count contiguous subarrays whose sum equals k (values may be negative).
#
# Approach: Prefix sums with a hash map of prefix -> occurrences; add count[prefix - k].
# Complexity: O(n) time, O(n) space

from collections import defaultdict


def subarray_sum(nums: list[int], k: int) -> int:
    count = defaultdict(int, {0: 1})
    prefix = total = 0
    for x in nums:
        prefix += x
        total += count[prefix - k]
        count[prefix] += 1
    return total


if __name__ == "__main__":
    assert subarray_sum([1, 1, 1], 2) == 2
    assert subarray_sum([1, 2, 3], 3) == 2
    assert subarray_sum([1, -1, 0], 0) == 3
    print("ok")
