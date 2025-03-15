# LeetCode 72 - Edit Distance
# Problem: Minimum number of insert/delete/replace operations to convert word1
#   into word2 (Levenshtein distance).
# Approach: dp[i][j] = dp[i-1][j-1] if equal else 1 + min(insert, delete, replace),
#   rolling one row.
# Time: O(m * n)  Space: O(n)


def min_distance(word1: str, word2: str) -> int:
    n = len(word2)
    dp = list(range(n + 1))
    for i, c1 in enumerate(word1, 1):
        diag, dp[0] = dp[0], i
        for j, c2 in enumerate(word2, 1):
            tmp = dp[j]
            dp[j] = diag if c1 == c2 else 1 + min(diag, dp[j], dp[j - 1])
            diag = tmp
    return dp[n]


if __name__ == "__main__":
    assert min_distance("horse", "ros") == 3
    assert min_distance("intention", "execution") == 5
    assert min_distance("", "abc") == 3
    print("ok")
