# LeetCode 494 - Target Sum
# Problem: Assign + or - to each number so the expression equals target; count
#   the number of such assignments.
# Approach: If P is the positive subset, sum(P) = (total + target) / 2. Count
#   subsets with that sum via 0/1 knapsack (iterate sums downwards).
# Time: O(n * S)  Space: O(S)


def find_target_sum_ways(nums: list[int], target: int) -> int:
    total = sum(nums)
    if abs(target) > total or (total + target) % 2:
        return 0
    s = (total + target) // 2
    dp = [1] + [0] * s
    for x in nums:
        for a in range(s, x - 1, -1):
            dp[a] += dp[a - x]
    return dp[s]


if __name__ == "__main__":
    assert find_target_sum_ways([1, 1, 1, 1, 1], 3) == 5
    assert find_target_sum_ways([1], 1) == 1
    assert find_target_sum_ways([0, 0, 1], 1) == 4
    print("ok")
