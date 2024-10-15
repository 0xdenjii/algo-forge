# LeetCode 14 - Longest Common Prefix
# Problem: Find the longest common prefix among an array of strings.
# Approach: The LCP of all strings equals the LCP of the lexicographic
#   min and max strings.
# Time: O(n * L)  Space: O(1)


def longest_common_prefix(strs: list[str]) -> str:
    if not strs:
        return ""
    lo, hi = min(strs), max(strs)
    i = 0
    while i < len(lo) and lo[i] == hi[i]:
        i += 1
    return lo[:i]


if __name__ == "__main__":
    assert longest_common_prefix(["flower", "flow", "flight"]) == "fl"
    assert longest_common_prefix(["dog", "racecar", "car"]) == ""
    assert longest_common_prefix(["a"]) == "a"
    print("ok")
