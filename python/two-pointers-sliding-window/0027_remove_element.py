"""
Problem: LeetCode 27
Title: 27. Remove Element

Statement:
    Remove all occurrences of val from nums in place; return the count k of
    remaining elements, stored in the first k slots.

Approach:
    Copy every non-val element to the write position.

Complexity: O(n) time, O(1) space.
"""

def remove_element(nums: list[int], val: int) -> int:
    w = 0
    for x in nums:
        if x != val:
            nums[w] = x
            w += 1
    return w


if __name__ == "__main__":
    a = [0, 1, 2, 2, 3, 0, 4, 2]
    k = remove_element(a, 2)
    assert k == 5 and sorted(a[:k]) == [0, 0, 1, 3, 4]
    print("ok")
