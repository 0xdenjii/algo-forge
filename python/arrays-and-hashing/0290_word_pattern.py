# Problem: LeetCode 290 - Word Pattern
#
# Given a pattern and a string of words, return True if there is a bijection
# between pattern letters and words.
#
# Approach: Two dictionaries enforcing the mapping in both directions.
# Complexity: O(n) time, O(n) space

def word_pattern(pattern: str, s: str) -> bool:
    words = s.split()
    if len(words) != len(pattern):
        return False
    p2w: dict[str, str] = {}
    w2p: dict[str, str] = {}
    for p, w in zip(pattern, words):
        if p2w.setdefault(p, w) != w or w2p.setdefault(w, p) != p:
            return False
    return True


if __name__ == "__main__":
    assert word_pattern("abba", "dog cat cat dog")
    assert not word_pattern("abba", "dog cat cat fish")
    assert not word_pattern("abba", "dog dog dog dog")
    print("ok")
