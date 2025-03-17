# LeetCode 312 - Burst Balloons
# Problem: Bursting balloon i earns nums[i-1]*nums[i]*nums[i+1] (neighbours at that
#   moment, 1 beyond the edges). Return the maximum coins from bursting all.
# Approach: Interval DP choosing the LAST balloon k burst in open interval (l, r):
#   dp[l][r] = max(dp[l][k] + dp[k][r] + a[l]*a[k]*a[r]).
# Time: O(n^3)  Space: O(n^2)


def max_coins(nums: list[int]) -> int:
    a = [1, *nums, 1]
    n = len(a)
    dp = [[0] * n for _ in range(n)]
    for gap in range(2, n):
        for l in range(n - gap):
            r = l + gap
            edge = a[l] * a[r]
            dp[l][r] = max(dp[l][k] + dp[k][r] + edge * a[k] for k in range(l + 1, r))
    return dp[0][n - 1]


if __name__ == "__main__":
    assert max_coins([3, 1, 5, 8]) == 167
    assert max_coins([1, 5]) == 10
    print("ok")
