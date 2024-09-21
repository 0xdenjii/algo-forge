# Problem: LeetCode 205 - Isomorphic Strings
#
# Two strings are isomorphic if characters of s can be consistently replaced
# to obtain t, with no two characters mapping to the same one.
#
# Approach: Compare the 'first-occurrence index' pattern of both strings.
# Complexity: O(n) time, O(k) space

def is_isomorphic(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False
    s_to_t: dict[str, str] = {}
    t_to_s: dict[str, str] = {}
    for a, b in zip(s, t):
        if s_to_t.setdefault(a, b) != b or t_to_s.setdefault(b, a) != a:
            return False
    return True


if __name__ == "__main__":
    assert is_isomorphic("egg", "add")
    assert not is_isomorphic("foo", "bar")
    assert is_isomorphic("paper", "title")
    assert not is_isomorphic("badc", "baba")
    print("ok")
