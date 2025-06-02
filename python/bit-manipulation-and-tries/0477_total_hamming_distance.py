# Problem: LeetCode 477 - Total Hamming Distance
#
# Return the sum of Hamming distances between all pairs of nums.
#
# Approach: Per bit, ones * zeros pairs differ at that bit.
# Complexity: O(32 n) time, O(1) space

def total_hamming_distance(nums: list[int]) -> int:
    n = len(nums)
    total = 0
    for bit in range(32):
        ones = sum((x >> bit) & 1 for x in nums)
        total += ones * (n - ones)
    return total


if __name__ == "__main__":
    assert total_hamming_distance([4, 14, 2]) == 6
    assert total_hamming_distance([4, 14, 4]) == 4
    print("ok")
