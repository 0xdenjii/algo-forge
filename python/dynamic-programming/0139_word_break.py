# LeetCode 139 - Word Break
# Problem: Given a string s and a dictionary of words, decide whether s can be
#   segmented into a space-separated sequence of dictionary words.
# Approach: ok[i] = any(ok[j] and s[j:i] in words) limited to max word length.
# Time: O(n * L)  Space: O(n)   (L = longest word length, slicing cost aside)


def word_break(s: str, word_dict: list[str]) -> bool:
    words = set(word_dict)
    max_len = max(map(len, words), default=0)
    ok = [True] + [False] * len(s)
    for i in range(1, len(s) + 1):
        for j in range(max(0, i - max_len), i):
            if ok[j] and s[j:i] in words:
                ok[i] = True
                break
    return ok[-1]


if __name__ == "__main__":
    assert word_break("leetcode", ["leet", "code"])
    assert word_break("applepenapple", ["apple", "pen"])
    assert not word_break("catsandog", ["cats", "dog", "sand", "and", "cat"])
    print("ok")
