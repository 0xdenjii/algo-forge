# Problem: LeetCode 128 - Longest Consecutive Sequence
#
# Given an unsorted array, return the length of the longest run of
# consecutive integers, in O(n) time.
#
# Approach: Put values in a set; only start counting from values whose predecessor is absent.
# Complexity: O(n) time, O(n) space

def longest_consecutive(nums: list[int]) -> int:
    values = set(nums)
    best = 0
    for v in values:
        if v - 1 not in values:
            end = v
            while end + 1 in values:
                end += 1
            best = max(best, end - v + 1)
    return best


if __name__ == "__main__":
    assert longest_consecutive([100, 4, 200, 1, 3, 2]) == 4
    assert longest_consecutive([0, 3, 7, 2, 5, 8, 4, 6, 0, 1]) == 9
    assert longest_consecutive([]) == 0
    print("ok")
