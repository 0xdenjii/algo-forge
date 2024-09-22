# Problem: LeetCode 287 - Find the Duplicate Number
#
# nums has n+1 integers in [1, n] with exactly one repeated value (possibly
# many times). Find it without modifying the array and with O(1) space.
#
# Approach: Treat i -> nums[i] as a linked list; Floyd's cycle detection finds the cycle entry.
# Complexity: O(n) time, O(1) space

def find_duplicate(nums: list[int]) -> int:
    slow = fast = nums[0]
    while True:
        slow = nums[slow]
        fast = nums[nums[fast]]
        if slow == fast:
            break
    slow = nums[0]
    while slow != fast:
        slow = nums[slow]
        fast = nums[fast]
    return slow


if __name__ == "__main__":
    assert find_duplicate([1, 3, 4, 2, 2]) == 2
    assert find_duplicate([3, 1, 3, 4, 2]) == 3
    assert find_duplicate([2, 2, 2, 2]) == 2
    print("ok")
