# Problem: LeetCode 208 - Implement Trie (Prefix Tree)
#
# Implement insert, search (exact word) and starts_with (prefix).
#
# Approach: Each node holds a dict of children and an end-of-word flag.
# Complexity: O(L) per op, O(total chars) space

from __future__ import annotations


class TrieNode:
    __slots__ = ("children", "is_end")

    def __init__(self) -> None:
        self.children: dict[str, TrieNode] = {}
        self.is_end = False


class Trie:
    def __init__(self) -> None:
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        node = self.root
        for c in word:
            node = node.children.setdefault(c, TrieNode())
        node.is_end = True

    def _walk(self, s: str) -> TrieNode | None:
        node = self.root
        for c in s:
            node = node.children.get(c)
            if node is None:
                return None
        return node

    def search(self, word: str) -> bool:
        node = self._walk(word)
        return node is not None and node.is_end

    def starts_with(self, prefix: str) -> bool:
        return self._walk(prefix) is not None


if __name__ == "__main__":
    t = Trie()
    t.insert("apple")
    assert t.search("apple") and not t.search("app") and t.starts_with("app")
    t.insert("app")
    assert t.search("app")
    print("ok")
