# LeetCode 32 - Longest Valid Parentheses
# Problem: Given a string of '(' and ')', return the length of the longest
#   well-formed parentheses substring.
# Approach: dp[i] = length of the longest valid substring ending at i. For ')':
#   "()" -> dp[i-2] + 2; "))" -> if s[i-dp[i-1]-1] == '(' extend and join prefix.
# Time: O(n)  Space: O(n)


def longest_valid_parentheses(s: str) -> int:
    dp = [0] * len(s)
    for i in range(1, len(s)):
        if s[i] != ")":
            continue
        if s[i - 1] == "(":
            dp[i] = (dp[i - 2] if i >= 2 else 0) + 2
        else:
            j = i - dp[i - 1] - 1
            if j >= 0 and s[j] == "(":
                dp[i] = dp[i - 1] + 2 + (dp[j - 1] if j >= 1 else 0)
    return max(dp, default=0)


if __name__ == "__main__":
    assert longest_valid_parentheses("(()") == 2
    assert longest_valid_parentheses(")()())") == 4
    assert longest_valid_parentheses("()(())") == 6
    assert longest_valid_parentheses("") == 0
    print("ok")
