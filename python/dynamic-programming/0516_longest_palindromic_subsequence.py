# LeetCode 516 - Longest Palindromic Subsequence
# Problem: Return the length of the longest subsequence of s that is a palindrome.
# Approach: dp[i][j] = dp[i+1][j-1] + 2 if s[i]==s[j] else max(dp[i+1][j], dp[i][j-1]);
#   iterate i downwards with a single rolling row.
# Time: O(n^2)  Space: O(n)


def longest_palindrome_subseq(s: str) -> int:
    n = len(s)
    dp = [0] * n
    for i in range(n - 1, -1, -1):
        dp[i] = 1
        diag = 0  # dp[i+1][j-1] from the previous row
        for j in range(i + 1, n):
            tmp = dp[j]
            dp[j] = diag + 2 if s[i] == s[j] else max(dp[j], dp[j - 1])
            diag = tmp
    return dp[-1] if n else 0


if __name__ == "__main__":
    assert longest_palindrome_subseq("bbbab") == 4
    assert longest_palindrome_subseq("cbbd") == 2
    assert longest_palindrome_subseq("a") == 1
    print("ok")
