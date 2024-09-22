# Problem: LeetCode 442 - Find All Duplicates in an Array
#
# nums has values in [1, n], each appearing once or twice; return those that
# appear twice, using O(1) extra space.
#
# Approach: Negate nums[v-1] on visit; if it is already negative, v is a duplicate.
# Complexity: O(n) time, O(1) extra space

def find_duplicates(nums: list[int]) -> list[int]:
    out: list[int] = []
    for v in nums:
        i = abs(v) - 1
        if nums[i] < 0:
            out.append(abs(v))
        else:
            nums[i] = -nums[i]
    return out


if __name__ == "__main__":
    assert sorted(find_duplicates([4, 3, 2, 7, 8, 2, 3, 1])) == [2, 3]
    assert find_duplicates([1]) == []
    print("ok")
