# Problem: LeetCode 169 - Majority Element
#
# Return the element that appears more than n/2 times (guaranteed to exist).
#
# Approach: Boyer-Moore voting: keep a candidate and a counter.
# Complexity: O(n) time, O(1) space

def majority_element(nums: list[int]) -> int:
    candidate, count = 0, 0
    for x in nums:
        if count == 0:
            candidate = x
        count += 1 if x == candidate else -1
    return candidate


if __name__ == "__main__":
    assert majority_element([3, 2, 3]) == 3
    assert majority_element([2, 2, 1, 1, 1, 2, 2]) == 2
    print("ok")
