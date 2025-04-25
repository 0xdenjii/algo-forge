# Problem: LeetCode 211 - Design Add and Search Words Data Structure
#
# Support add_word(word) and search(pattern) where '.' matches any letter.
#
# Approach: Trie with DFS that branches over all children on '.'.
# Complexity: add O(L); search O(26^dots * L) worst, O(total chars) space

class WordDictionary:
    def __init__(self) -> None:
        self.root: dict = {}

    def add_word(self, word: str) -> None:
        node = self.root
        for c in word:
            node = node.setdefault(c, {})
        node["$"] = True

    def search(self, word: str) -> bool:
        def dfs(node: dict, i: int) -> bool:
            if i == len(word):
                return "$" in node
            c = word[i]
            if c == ".":
                return any(k != "$" and dfs(child, i + 1) for k, child in node.items())
            child = node.get(c)
            return child is not None and dfs(child, i + 1)

        return dfs(self.root, 0)


if __name__ == "__main__":
    wd = WordDictionary()
    for w in ("bad", "dad", "mad"):
        wd.add_word(w)
    assert not wd.search("pad") and wd.search("bad") and wd.search(".ad") and wd.search("b..")
    assert not wd.search("b...")
    print("ok")
