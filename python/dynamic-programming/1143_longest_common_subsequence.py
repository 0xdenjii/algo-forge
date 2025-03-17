# LeetCode 1143 - Longest Common Subsequence
# Problem: Return the length of the longest subsequence common to two strings.
# Approach: dp[i][j] = dp[i-1][j-1]+1 if chars match else max(dp[i-1][j], dp[i][j-1]);
#   keep a single rolling row over the shorter string.
# Time: O(m * n)  Space: O(min(m, n))


def longest_common_subsequence(a: str, b: str) -> int:
    if len(a) < len(b):
        a, b = b, a
    dp = [0] * (len(b) + 1)
    for ca in a:
        diag = 0
        for j, cb in enumerate(b, 1):
            tmp = dp[j]
            dp[j] = diag + 1 if ca == cb else max(dp[j], dp[j - 1])
            diag = tmp
    return dp[-1]


if __name__ == "__main__":
    assert longest_common_subsequence("abcde", "ace") == 3
    assert longest_common_subsequence("abc", "def") == 0
    assert longest_common_subsequence("abc", "abc") == 3
    print("ok")
