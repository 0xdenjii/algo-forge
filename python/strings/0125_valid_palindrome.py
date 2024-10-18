# LeetCode 125 - Valid Palindrome
# Problem: After lowercasing and removing non-alphanumeric characters,
#   is the string a palindrome?
# Approach: Two pointers moving inward, skipping non-alphanumerics.
# Time: O(n)  Space: O(1)


def is_palindrome(s: str) -> bool:
    i, j = 0, len(s) - 1
    while i < j:
        if not s[i].isalnum():
            i += 1
        elif not s[j].isalnum():
            j -= 1
        elif s[i].lower() != s[j].lower():
            return False
        else:
            i += 1
            j -= 1
    return True


if __name__ == "__main__":
    assert is_palindrome("A man, a plan, a canal: Panama")
    assert not is_palindrome("race a car")
    assert is_palindrome(" ")
    print("ok")
