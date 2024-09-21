# Problem: LeetCode 229 - Majority Element II
#
# Find all elements that appear more than n/3 times.
#
# Approach: Extended Boyer-Moore with two candidates, then verify counts.
# Complexity: O(n) time, O(1) space

def majority_element(nums: list[int]) -> list[int]:
    c1, c2, n1, n2 = None, None, 0, 0
    for x in nums:
        if x == c1:
            n1 += 1
        elif x == c2:
            n2 += 1
        elif n1 == 0:
            c1, n1 = x, 1
        elif n2 == 0:
            c2, n2 = x, 1
        else:
            n1 -= 1
            n2 -= 1
    return [c for c in (c1, c2) if c is not None and nums.count(c) > len(nums) // 3]


if __name__ == "__main__":
    assert majority_element([3, 2, 3]) == [3]
    assert majority_element([1]) == [1]
    assert sorted(majority_element([1, 2])) == [1, 2]
    print("ok")
