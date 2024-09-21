# Problem: LeetCode 41 - First Missing Positive
#
# Return the smallest missing positive integer from an unsorted array in
# O(n) time and O(1) extra space.
#
# Approach: Cyclic placement: swap each value v in [1, n] to index v-1; first mismatch is the answer.
# Complexity: O(n) time, O(1) space (mutates input)

def first_missing_positive(nums: list[int]) -> int:
    n = len(nums)
    for i in range(n):
        while 1 <= nums[i] <= n and nums[nums[i] - 1] != nums[i]:
            j = nums[i] - 1
            nums[i], nums[j] = nums[j], nums[i]
    for i, v in enumerate(nums):
        if v != i + 1:
            return i + 1
    return n + 1


if __name__ == "__main__":
    assert first_missing_positive([1, 2, 0]) == 3
    assert first_missing_positive([3, 4, -1, 1]) == 2
    assert first_missing_positive([7, 8, 9, 11, 12]) == 1
    print("ok")
