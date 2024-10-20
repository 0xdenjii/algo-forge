# LeetCode 205 - Isomorphic Strings
# Problem: Can chars of s be replaced (one-to-one) to obtain t?
# Approach: Keep forward and backward maps; any conflict breaks the
#   bijection.
# Time: O(n)  Space: O(alphabet)


def is_isomorphic(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False
    ms: dict[str, str] = {}
    mt: dict[str, str] = {}
    for a, b in zip(s, t):
        if ms.setdefault(a, b) != b or mt.setdefault(b, a) != a:
            return False
    return True


if __name__ == "__main__":
    assert is_isomorphic("egg", "add")
    assert not is_isomorphic("foo", "bar")
    assert is_isomorphic("paper", "title")
    assert not is_isomorphic("badc", "baba")
    print("ok")
