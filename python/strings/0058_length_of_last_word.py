# LeetCode 58 - Length of Last Word
# Problem: Return the length of the last word in a string of words and
#   spaces (possibly trailing spaces).
# Approach: Skip trailing spaces from the right, then count letters.
# Time: O(n)  Space: O(1)


def length_of_last_word(s: str) -> int:
    i = len(s) - 1
    while i >= 0 and s[i] == " ":
        i -= 1
    j = i
    while j >= 0 and s[j] != " ":
        j -= 1
    return i - j


if __name__ == "__main__":
    assert length_of_last_word("Hello World") == 5
    assert length_of_last_word("   fly me   to   the moon  ") == 4
    assert length_of_last_word("luffy is still joyboy") == 6
    print("ok")
