# Problem: LeetCode 525 - Contiguous Array
#
# In a binary array, find the maximum length of a contiguous subarray with an
# equal number of 0s and 1s.
#
# Approach: Map 0 -> -1, track first index of each running balance; equal balance = zero-sum span.
# Complexity: O(n) time, O(n) space

def find_max_length(nums: list[int]) -> int:
    first = {0: -1}
    balance = best = 0
    for i, x in enumerate(nums):
        balance += 1 if x else -1
        if balance in first:
            best = max(best, i - first[balance])
        else:
            first[balance] = i
    return best


if __name__ == "__main__":
    assert find_max_length([0, 1]) == 2
    assert find_max_length([0, 1, 0]) == 2
    assert find_max_length([0, 0, 1, 0, 0, 0, 1, 1]) == 6
    print("ok")
