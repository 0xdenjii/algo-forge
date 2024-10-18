# LeetCode 151 - Reverse Words in a String
# Problem: Reverse word order, collapsing multiple spaces and trimming.
# Approach: str.split() with no args handles all whitespace runs.
# Time: O(n)  Space: O(n)


def reverse_words(s: str) -> str:
    return " ".join(reversed(s.split()))


if __name__ == "__main__":
    assert reverse_words("the sky is blue") == "blue is sky the"
    assert reverse_words("  hello world  ") == "world hello"
    assert reverse_words("a good   example") == "example good a"
    print("ok")
