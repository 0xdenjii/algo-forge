# LeetCode 3 - Longest Substring Without Repeating Characters
# Problem: Given a string s, return the length of the longest substring
#   that contains no repeated characters.
# Approach: Sliding window; remember last index of each char and jump the
#   left edge past a repeated char.
# Time: O(n)  Space: O(min(n, alphabet))


def length_of_longest_substring(s: str) -> int:
    last: dict[str, int] = {}
    left = best = 0
    for right, ch in enumerate(s):
        if last.get(ch, -1) >= left:
            left = last[ch] + 1
        last[ch] = right
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    assert length_of_longest_substring("abcabcbb") == 3
    assert length_of_longest_substring("bbbbb") == 1
    assert length_of_longest_substring("pwwkew") == 3
    assert length_of_longest_substring("") == 0
    assert length_of_longest_substring("abba") == 2
    print("ok")
