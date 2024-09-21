# Problem: LeetCode 49 - Group Anagrams
#
# Group the strings of an array so that anagrams end up together.
#
# Approach: Key each word by its 26-letter count tuple (avoids sorting each word).
# Complexity: O(n * m) time for n words of length m, O(n * m) space

from collections import defaultdict


def group_anagrams(words: list[str]) -> list[list[str]]:
    groups: dict[tuple[int, ...], list[str]] = defaultdict(list)
    for w in words:
        counts = [0] * 26
        for ch in w:
            counts[ord(ch) - ord("a")] += 1
        groups[tuple(counts)].append(w)
    return list(groups.values())


if __name__ == "__main__":
    out = group_anagrams(["eat", "tea", "tan", "ate", "nat", "bat"])
    assert sorted(sorted(g) for g in out) == [["ate", "eat", "tea"], ["bat"], ["nat", "tan"]]
    print("ok")
