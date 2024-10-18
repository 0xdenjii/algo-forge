# LeetCode 139 - Word Break
# Problem: Can s be segmented into a sequence of dictionary words?
# Approach: dp[i] is True if s[:i] is segmentable; try word lengths only.
# Time: O(n * L * k), L = distinct word lengths  Space: O(n)


def word_break(s: str, word_dict: list[str]) -> bool:
    words = set(word_dict)
    lengths = {len(w) for w in words}
    dp = [True] + [False] * len(s)
    for i in range(1, len(s) + 1):
        dp[i] = any(l <= i and dp[i - l] and s[i - l:i] in words for l in lengths)
    return dp[-1]


if __name__ == "__main__":
    assert word_break("leetcode", ["leet", "code"])
    assert word_break("applepenapple", ["apple", "pen"])
    assert not word_break("catsandog", ["cats", "dog", "sand", "and", "cat"])
    print("ok")
