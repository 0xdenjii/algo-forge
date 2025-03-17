# LeetCode 221 - Maximal Square
# Problem: In a binary matrix of '0'/'1' characters, find the area of the largest
#   square containing only '1's.
# Approach: side[i][j] = 1 + min(top, left, top-left) for '1' cells; rolling row.
# Time: O(m * n)  Space: O(n)


def maximal_square(matrix: list[list[str]]) -> int:
    n = len(matrix[0])
    dp = [0] * (n + 1)
    best = 0
    for row in matrix:
        diag = 0
        for j in range(1, n + 1):
            tmp = dp[j]
            dp[j] = 1 + min(dp[j], dp[j - 1], diag) if row[j - 1] == "1" else 0
            best = max(best, dp[j])
            diag = tmp
    return best * best


if __name__ == "__main__":
    grid = [list("10100"), list("10111"), list("11111"), list("10010")]
    assert maximal_square(grid) == 4
    assert maximal_square([list("01"), list("10")]) == 1
    assert maximal_square([["0"]]) == 0
    print("ok")
