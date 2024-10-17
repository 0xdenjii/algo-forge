# LeetCode 49 - Group Anagrams
# Problem: Group strings that are anagrams of one another.
# Approach: Use a 26-count tuple as a canonical dictionary key.
# Time: O(n * k)  Space: O(n * k)
from collections import defaultdict


def group_anagrams(strs: list[str]) -> list[list[str]]:
    groups: dict[tuple[int, ...], list[str]] = defaultdict(list)
    for s in strs:
        counts = [0] * 26
        for ch in s:
            counts[ord(ch) - 97] += 1
        groups[tuple(counts)].append(s)
    return list(groups.values())


if __name__ == "__main__":
    res = group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"])
    assert sorted(map(sorted, res)) == [["ate", "eat", "tea"], ["bat"], ["nat", "tan"]]
    print("ok")
