# LeetCode 115 - Distinct Subsequences
# Problem: Count the distinct subsequences of s that equal t.
# Approach: dp[j] = ways to form t[:j]; for each char of s, iterate j downwards and
#   add dp[j-1] when s char == t[j-1].
# Time: O(m * n)  Space: O(n)


def num_distinct(s: str, t: str) -> int:
    dp = [1] + [0] * len(t)
    for c in s:
        for j in range(len(t), 0, -1):
            if t[j - 1] == c:
                dp[j] += dp[j - 1]
    return dp[-1]


if __name__ == "__main__":
    assert num_distinct("rabbbit", "rabbit") == 3
    assert num_distinct("babgbag", "bag") == 5
    assert num_distinct("abc", "") == 1
    print("ok")
