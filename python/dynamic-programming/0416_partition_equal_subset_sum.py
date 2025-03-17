# LeetCode 416 - Partition Equal Subset Sum
# Problem: Decide whether an array of positive integers can be split into two
#   subsets with equal sums.
# Approach: 0/1 subset-sum to total/2. Uses a Python int as a bitset: bit k set
#   means sum k is reachable; shifting ORs in each number.
# Time: O(n * S / w)  Space: O(S) bits


def can_partition(nums: list[int]) -> bool:
    total = sum(nums)
    if total % 2:
        return False
    target = total // 2
    reach = 1
    for x in nums:
        reach |= reach << x
    return bool(reach >> target & 1)


if __name__ == "__main__":
    assert can_partition([1, 5, 11, 5])
    assert not can_partition([1, 2, 3, 5])
    assert can_partition([2, 2])
    print("ok")
