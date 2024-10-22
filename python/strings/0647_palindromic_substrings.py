# LeetCode 647 - Palindromic Substrings
# Problem: Count substrings of s that are palindromes.
# Approach: Expand from every one of the 2n-1 centers, counting matches.
# Time: O(n^2)  Space: O(1)


def count_substrings(s: str) -> int:
    n, total = len(s), 0
    for c in range(2 * n - 1):
        l, r = c // 2, c // 2 + c % 2
        while l >= 0 and r < n and s[l] == s[r]:
            total += 1
            l -= 1
            r += 1
    return total


if __name__ == "__main__":
    assert count_substrings("abc") == 3
    assert count_substrings("aaa") == 6
    print("ok")
