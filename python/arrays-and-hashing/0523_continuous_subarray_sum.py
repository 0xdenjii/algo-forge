# Problem: LeetCode 523 - Continuous Subarray Sum
#
# Return True if nums has a subarray of length >= 2 whose sum is a multiple of k.
#
# Approach: Store the first index of each prefix residue mod k; a repeat at distance >= 2 wins.
# Complexity: O(n) time, O(min(n, k)) space

def check_subarray_sum(nums: list[int], k: int) -> bool:
    first = {0: -1}
    prefix = 0
    for i, x in enumerate(nums):
        prefix = (prefix + x) % k
        if prefix in first:
            if i - first[prefix] >= 2:
                return True
        else:
            first[prefix] = i
    return False


if __name__ == "__main__":
    assert check_subarray_sum([23, 2, 4, 6, 7], 6)
    assert check_subarray_sum([23, 2, 6, 4, 7], 6)
    assert not check_subarray_sum([23, 2, 6, 4, 7], 13)
    print("ok")
