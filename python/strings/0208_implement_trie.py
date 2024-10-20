# LeetCode 208 - Implement Trie (Prefix Tree)
# Problem: Support insert(word), search(word), startsWith(prefix).
# Approach: Nested dict nodes with an end-of-word sentinel key.
# Time: O(L) per operation  Space: O(total chars inserted)

END = "$"


class Trie:
    def __init__(self) -> None:
        self.root: dict = {}

    def insert(self, word: str) -> None:
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node[END] = True

    def _walk(self, s: str):
        node = self.root
        for ch in s:
            node = node.get(ch)
            if node is None:
                return None
        return node

    def search(self, word: str) -> bool:
        node = self._walk(word)
        return node is not None and END in node

    def starts_with(self, prefix: str) -> bool:
        return self._walk(prefix) is not None


if __name__ == "__main__":
    t = Trie()
    t.insert("apple")
    assert t.search("apple") and not t.search("app") and t.starts_with("app")
    t.insert("app")
    assert t.search("app")
    print("ok")
