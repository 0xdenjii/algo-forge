# Problem: LeetCode 14 - Longest Common Prefix (trie)
#
# Return the longest common prefix among a list of strings.
#
# Approach: Insert all words into a trie; walk from root while node has one child and
#   is not a word end.
# Complexity: O(total chars) time and space

def longest_common_prefix(strs: list[str]) -> str:
    if not strs:
        return ""
    root: dict = {}
    for s in strs:
        node = root
        for c in s:
            node = node.setdefault(c, {})
        node["$"] = True
    prefix = []
    node = root
    while len(node) == 1 and "$" not in node:
        c, node = next(iter(node.items()))
        prefix.append(c)
    return "".join(prefix)


if __name__ == "__main__":
    assert longest_common_prefix(["flower", "flow", "flight"]) == "fl"
    assert longest_common_prefix(["dog", "racecar", "car"]) == ""
    assert longest_common_prefix(["ab", "a"]) == "a"
    print("ok")
