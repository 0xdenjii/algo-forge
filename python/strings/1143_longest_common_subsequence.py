# LeetCode 1143 - Longest Common Subsequence
# Problem: Length of the longest subsequence common to both strings.
# Approach: dp[i][j] = dp[i-1][j-1]+1 on match else max of neighbours;
#   keep one row.
# Time: O(n * m)  Space: O(m)


def longest_common_subsequence(a: str, b: str) -> int:
    prev = [0] * (len(b) + 1)
    for ca in a:
        cur = [0]
        for j, cb in enumerate(b, 1):
            cur.append(prev[j - 1] + 1 if ca == cb else max(prev[j], cur[j - 1]))
        prev = cur
    return prev[-1]


if __name__ == "__main__":
    assert longest_common_subsequence("abcde", "ace") == 3
    assert longest_common_subsequence("abc", "abc") == 3
    assert longest_common_subsequence("abc", "def") == 0
    print("ok")
