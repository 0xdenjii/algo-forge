# Problem: LeetCode 648 - Replace Words
#
# Replace each word in a sentence by the shortest dictionary root that is its prefix.
#
# Approach: Insert roots in a trie; for each word walk until a root end or mismatch.
# Complexity: O(total chars) time and space

def replace_words(dictionary: list[str], sentence: str) -> str:
    root: dict = {}
    for w in dictionary:
        node = root
        for c in w:
            node = node.setdefault(c, {})
        node["$"] = w

    def shortest_root(word: str) -> str:
        node = root
        for c in word:
            if c not in node:
                break
            node = node[c]
            if "$" in node:
                return node["$"]
        return word

    return " ".join(shortest_root(w) for w in sentence.split())


if __name__ == "__main__":
    out = replace_words(["cat", "bat", "rat"], "the cattle was rattled by the battery")
    assert out == "the cat was rat by the bat"
    print("ok")
