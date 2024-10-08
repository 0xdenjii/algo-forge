"""
Problem: LeetCode 287
Title: 287. Find the Duplicate Number

Statement:
    An array of n + 1 integers in [1, n] has exactly one repeated value; find it
    without modifying the array and using O(1) extra space.

Approach:
    Treat i -> nums[i] as a linked list; the duplicate is the cycle entry.

Complexity: O(n) time, O(1) space.
"""

def find_duplicate(nums: list[int]) -> int:
    slow = fast = nums[0]
    while True:
        slow = nums[slow]
        fast = nums[nums[fast]]
        if slow == fast:
            break
    slow = nums[0]
    while slow != fast:
        slow, fast = nums[slow], nums[fast]
    return slow


if __name__ == "__main__":
    assert find_duplicate([1, 3, 4, 2, 2]) == 2
    assert find_duplicate([3, 1, 3, 4, 2]) == 3
    assert find_duplicate([3, 3, 3, 3, 3]) == 3
    print("ok")
