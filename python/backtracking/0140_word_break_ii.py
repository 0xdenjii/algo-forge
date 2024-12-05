"""
LeetCode 140 - Word Break II
Problem: Given string s and a dictionary, insert spaces to build sentences
where every word is in the dictionary. Return all such sentences.
Approach: Backtracking memoized on the start index (all sentences for the
suffix s[i:]), only trying word lengths up to the longest dictionary word.
Time: O(n^2 + output)  Space: O(n * output)
"""
from functools import lru_cache
from typing import List, Tuple


def word_break(s: str, word_dict: List[str]) -> List[str]:
    words = set(word_dict)
    max_len = max(map(len, words), default=0)

    @lru_cache(maxsize=None)
    def dfs(i: int) -> Tuple[str, ...]:
        if i == len(s):
            return ("",)
        res = []
        for j in range(i + 1, min(len(s), i + max_len) + 1):
            w = s[i:j]
            if w in words:
                for tail in dfs(j):
                    res.append(w + (" " + tail if tail else ""))
        return tuple(res)

    return list(dfs(0))


if __name__ == "__main__":
    assert sorted(word_break("catsanddog", ["cat", "cats", "and", "sand", "dog"])) == [
        "cat sand dog", "cats and dog"]
    assert sorted(word_break("pineapplepenapple",
                             ["apple", "pen", "applepen", "pine", "pineapple"])) == [
        "pine apple pen apple", "pine applepen apple", "pineapple pen apple"]
    assert word_break("catsandog", ["cats", "dog", "sand", "and", "cat"]) == []
    print("ok")
