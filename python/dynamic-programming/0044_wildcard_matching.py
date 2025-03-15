# LeetCode 44 - Wildcard Matching
# Problem: Full-string match of s against p where '?' matches one char and '*'
#   matches any sequence (including empty).
# Approach: dp over pattern prefixes x string prefixes with a rolling row:
#   '*' -> dp[j-1] (empty) or prev[j] (consume a char); '?'/equal -> prev[j-1].
# Time: O(m * n)  Space: O(n)


def is_match(s: str, p: str) -> bool:
    prev = [True] + [False] * len(s)
    for pc in p:
        cur = [prev[0] and pc == "*"] + [False] * len(s)
        for j, sc in enumerate(s, 1):
            if pc == "*":
                cur[j] = cur[j - 1] or prev[j]
            else:
                cur[j] = prev[j - 1] and pc in (sc, "?")
        prev = cur
    return prev[-1]


if __name__ == "__main__":
    assert not is_match("aa", "a")
    assert is_match("aa", "*")
    assert not is_match("cb", "?a")
    assert is_match("adceb", "*a*b")
    assert not is_match("acdcb", "a*c?b")
    print("ok")
