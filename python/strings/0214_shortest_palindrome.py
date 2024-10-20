# LeetCode 214 - Shortest Palindrome
# Problem: Add the fewest characters to the FRONT of s to make it a
#   palindrome; return the result.
# Approach: Longest palindromic prefix = last value of the KMP prefix
#   function of s + "#" + reversed(s).
# Time: O(n)  Space: O(n)


def shortest_palindrome(s: str) -> str:
    t = s + "#" + s[::-1]
    pi = [0] * len(t)
    for i in range(1, len(t)):
        k = pi[i - 1]
        while k and t[i] != t[k]:
            k = pi[k - 1]
        if t[i] == t[k]:
            k += 1
        pi[i] = k
    return s[pi[-1]:][::-1] + s


if __name__ == "__main__":
    assert shortest_palindrome("aacecaaa") == "aaacecaaa"
    assert shortest_palindrome("abcd") == "dcbabcd"
    assert shortest_palindrome("") == ""
    print("ok")
