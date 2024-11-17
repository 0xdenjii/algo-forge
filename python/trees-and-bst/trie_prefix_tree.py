# LeetCode 208 - Implement Trie (Prefix Tree)
# Problem: Implement insert(word), search(word) and starts_with(prefix).
# Approach: Each node is a dict of children plus an end-of-word flag.
# Time: O(L) per op  Space: O(total characters)

from __future__ import annotations


class TrieNode:
    __slots__ = ("children", "end")

    def __init__(self) -> None:
        self.children: dict[str, TrieNode] = {}
        self.end = False


class Trie:
    def __init__(self) -> None:
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        node = self.root
        for ch in word:
            node = node.children.setdefault(ch, TrieNode())
        node.end = True

    def _walk(self, s: str) -> TrieNode | None:
        node = self.root
        for ch in s:
            node = node.children.get(ch)
            if node is None:
                return None
        return node

    def search(self, word: str) -> bool:
        node = self._walk(word)
        return node is not None and node.end

    def starts_with(self, prefix: str) -> bool:
        return self._walk(prefix) is not None


if __name__ == "__main__":
    t = Trie()
    t.insert("apple")
    assert t.search("apple") and not t.search("app") and t.starts_with("app")
    t.insert("app")
    assert t.search("app")
    print("ok")
