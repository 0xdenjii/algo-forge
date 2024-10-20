# LeetCode 290 - Word Pattern
# Problem: Does string s (space-separated words) follow the letter
#   pattern with a bijection between letters and words?
# Approach: A bijection exists iff #letters == #words == #distinct pairs.
# Time: O(n)  Space: O(n)


def word_pattern(pattern: str, s: str) -> bool:
    words = s.split()
    if len(words) != len(pattern):
        return False
    return len(set(pattern)) == len(set(words)) == len(set(zip(pattern, words)))


if __name__ == "__main__":
    assert word_pattern("abba", "dog cat cat dog")
    assert not word_pattern("abba", "dog cat cat fish")
    assert not word_pattern("aaaa", "dog cat cat dog")
    assert not word_pattern("abba", "dog dog dog dog")
    print("ok")
