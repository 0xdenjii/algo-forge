# Problem: LeetCode 219 - Contains Duplicate II
#
# Return True if there are distinct indices i, j with nums[i] == nums[j]
# and |i - j| <= k.
#
# Approach: Store the last index of each value; compare distance on repeat.
# Complexity: O(n) time, O(n) space

def contains_nearby_duplicate(nums: list[int], k: int) -> bool:
    last: dict[int, int] = {}
    for i, x in enumerate(nums):
        if x in last and i - last[x] <= k:
            return True
        last[x] = i
    return False


if __name__ == "__main__":
    assert contains_nearby_duplicate([1, 2, 3, 1], 3)
    assert contains_nearby_duplicate([1, 0, 1, 1], 1)
    assert not contains_nearby_duplicate([1, 2, 3, 1, 2, 3], 2)
    print("ok")
