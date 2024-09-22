# Problem: LeetCode 448 - Find All Numbers Disappeared in an Array
#
# nums has n values in [1, n]; return every value in that range that is missing.
#
# Approach: Mark presence by negating nums[v-1]; indices still positive are missing.
# Complexity: O(n) time, O(1) extra space

def find_disappeared_numbers(nums: list[int]) -> list[int]:
    for v in nums:
        i = abs(v) - 1
        nums[i] = -abs(nums[i])
    return [i + 1 for i, v in enumerate(nums) if v > 0]


if __name__ == "__main__":
    assert find_disappeared_numbers([4, 3, 2, 7, 8, 2, 3, 1]) == [5, 6]
    assert find_disappeared_numbers([1, 1]) == [2]
    print("ok")
