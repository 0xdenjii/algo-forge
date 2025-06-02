# Problem: LeetCode 472 - Concatenated Words
#
# Return all words formed by concatenating at least two shorter words from the list.
#
# Approach: Trie of all words; for each word, memoized DFS that splits at word-end nodes,
#   tracking the maximum number of pieces.
# Complexity: O(n L^2) time, O(total chars) space

from functools import lru_cache


def find_all_concatenated_words(words: list[str]) -> list[str]:
    root: dict = {}
    for w in words:
        if w:
            node = root
            for c in w:
                node = node.setdefault(c, {})
            node["$"] = True

    def is_concat(word: str) -> bool:
        @lru_cache(maxsize=None)
        def pieces(start: int) -> int:
            # max number of dictionary words covering word[start:], -1 if impossible
            if start == len(word):
                return 0
            node, best = root, -1
            for i in range(start, len(word)):
                node = node.get(word[i])
                if node is None:
                    break
                if "$" in node:
                    rest = pieces(i + 1)
                    if rest >= 0:
                        best = max(best, rest + 1)
            return best

        return pieces(0) >= 2

    return [w for w in words if w and is_concat(w)]


if __name__ == "__main__":
    words = ["cat", "cats", "catsdogcats", "dog", "dogcatsdog", "hippopotamuses", "rat", "ratcatdogcat"]
    assert sorted(find_all_concatenated_words(words)) == ["catsdogcats", "dogcatsdog", "ratcatdogcat"]
    print("ok")
