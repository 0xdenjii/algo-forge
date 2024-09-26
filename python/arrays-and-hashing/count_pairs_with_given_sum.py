# Problem: Count Pairs With Given Sum
#
# Count index pairs i < j with nums[i] + nums[j] == target (classic hashing
# exercise, e.g. GeeksforGeeks).
#
# Approach: Single pass; add frequency of the complement seen so far, then record the value.
# Complexity: O(n) time, O(n) space

from collections import defaultdict


def count_pairs(nums: list[int], target: int) -> int:
    freq: dict[int, int] = defaultdict(int)
    total = 0
    for x in nums:
        total += freq[target - x]
        freq[x] += 1
    return total


if __name__ == "__main__":
    assert count_pairs([1, 5, 7, -1], 6) == 2
    assert count_pairs([1, 5, 7, -1, 5], 6) == 3
    assert count_pairs([1, 1, 1, 1], 2) == 6
    print("ok")
