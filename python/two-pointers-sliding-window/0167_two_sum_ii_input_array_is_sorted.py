"""
Problem: LeetCode 167
Title: 167. Two Sum II - Input Array Is Sorted

Statement:
    Given a 1-indexed sorted array and a target, return the indices of the two
    numbers that add up to target. Exactly one solution exists.

Approach:
    Converging pointers: move left up if sum too small, right down if too big.

Complexity: O(n) time, O(1) space.
"""

def two_sum(numbers: list[int], target: int) -> list[int]:
    lo, hi = 0, len(numbers) - 1
    while lo < hi:
        s = numbers[lo] + numbers[hi]
        if s == target:
            return [lo + 1, hi + 1]
        if s < target:
            lo += 1
        else:
            hi -= 1
    return []


if __name__ == "__main__":
    assert two_sum([2, 7, 11, 15], 9) == [1, 2]
    assert two_sum([2, 3, 4], 6) == [1, 3]
    assert two_sum([-1, 0], -1) == [1, 2]
    print("ok")
