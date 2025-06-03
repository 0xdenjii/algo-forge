# Problem: LeetCode 720 - Longest Word in Dictionary
#
# Return the longest word buildable one character at a time from other words
# in the list (ties broken lexicographically).
#
# Approach: Trie insert, then DFS only through nodes that end a word.
# Complexity: O(total chars) time and space

def longest_word(words: list[str]) -> str:
    root: dict = {}
    for w in words:
        node = root
        for c in w:
            node = node.setdefault(c, {})
        node["$"] = w
    best = ""
    stack = [root]
    while stack:
        node = stack.pop()
        for c, child in node.items():
            if c != "$" and "$" in child:
                w = child["$"]
                if len(w) > len(best) or (len(w) == len(best) and w < best):
                    best = w
                stack.append(child)
    return best


if __name__ == "__main__":
    assert longest_word(["w", "wo", "wor", "worl", "world"]) == "world"
    assert longest_word(["a", "banana", "app", "appl", "ap", "apply", "apple"]) == "apple"
    print("ok")
