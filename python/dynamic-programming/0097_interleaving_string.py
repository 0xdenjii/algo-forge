# LeetCode 97 - Interleaving String
# Problem: Decide whether s3 is formed by interleaving s1 and s2 while keeping the
#   relative order of characters from each.
# Approach: dp[j] = s3[:i+j] is an interleave of s1[:i], s2[:j]; rolling row.
# Time: O(m * n)  Space: O(n)


def is_interleave(s1: str, s2: str, s3: str) -> bool:
    m, n = len(s1), len(s2)
    if m + n != len(s3):
        return False
    dp = [False] * (n + 1)
    for i in range(m + 1):
        for j in range(n + 1):
            if i == 0 and j == 0:
                dp[j] = True
                continue
            from_s1 = i > 0 and dp[j] and s1[i - 1] == s3[i + j - 1]
            from_s2 = j > 0 and dp[j - 1] and s2[j - 1] == s3[i + j - 1]
            dp[j] = from_s1 or from_s2
    return dp[n]


if __name__ == "__main__":
    assert is_interleave("aabcc", "dbbca", "aadbbcbcac")
    assert not is_interleave("aabcc", "dbbca", "aadbbbaccc")
    assert is_interleave("", "", "")
    print("ok")
