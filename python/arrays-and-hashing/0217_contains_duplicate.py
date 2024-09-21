# Problem: LeetCode 217 - Contains Duplicate
#
# Return True if any value appears at least twice in the array.
#
# Approach: Track seen values in a set, stop at the first repeat.
# Complexity: O(n) time, O(n) space

def contains_duplicate(nums: list[int]) -> bool:
    seen: set[int] = set()
    for x in nums:
        if x in seen:
            return True
        seen.add(x)
    return False


if __name__ == "__main__":
    assert contains_duplicate([1, 2, 3, 1])
    assert not contains_duplicate([1, 2, 3, 4])
    assert not contains_duplicate([])
    print("ok")
