# Problem: LeetCode 974 - Subarray Sums Divisible by K
#
# Count non-empty subarrays whose sum is divisible by k.
#
# Approach: Count prefix sums by residue mod k; pairs with equal residue form valid subarrays.
# Complexity: O(n) time, O(k) space

def subarrays_div_by_k(nums: list[int], k: int) -> int:
    counts = [0] * k
    counts[0] = 1
    prefix = total = 0
    for x in nums:
        prefix = (prefix + x) % k  # Python's % is non-negative for k > 0
        total += counts[prefix]
        counts[prefix] += 1
    return total


if __name__ == "__main__":
    assert subarrays_div_by_k([4, 5, 0, -2, -3, 1], 5) == 7
    assert subarrays_div_by_k([5], 9) == 0
    print("ok")
