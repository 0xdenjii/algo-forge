# Problem: LeetCode 78 - Subsets (bitmask enumeration)
#
# Return all subsets of a list of distinct integers.
#
# Approach: Each mask in [0, 2^n) selects a subset: element i included iff bit i set.
# Complexity: O(n 2^n) time, O(n 2^n) output

def subsets(nums: list[int]) -> list[list[int]]:
    n = len(nums)
    return [[nums[i] for i in range(n) if mask >> i & 1] for mask in range(1 << n)]


if __name__ == "__main__":
    res = subsets([1, 2, 3])
    assert len(res) == 8 and sorted(map(sorted, res)) == sorted(
        [[], [1], [2], [3], [1, 2], [1, 3], [2, 3], [1, 2, 3]])
    print("ok")
