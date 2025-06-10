# Problem: LeetCode 1268 - Search Suggestions System
#
# After each typed character of searchWord, return up to three lexicographically
# smallest products having the typed prefix.
#
# Approach: Trie where each node caches its top-3 words (insert products in sorted order).
# Complexity: O(total chars) build, O(L) query

def suggested_products(products: list[str], search_word: str) -> list[list[str]]:
    root: dict = {}
    for p in sorted(products):
        node = root
        for c in p:
            node = node.setdefault(c, {"#": []})
            if len(node["#"]) < 3:
                node["#"].append(p)
    result: list[list[str]] = []
    node = root
    for c in search_word:
        node = node.get(c) if node is not None else None
        result.append(node["#"] if node else [])
    return result


if __name__ == "__main__":
    out = suggested_products(["mobile", "mouse", "moneypot", "monitor", "mousepad"], "mouse")
    assert out == [["mobile", "moneypot", "monitor"], ["mobile", "moneypot", "monitor"],
                   ["mouse", "mousepad"], ["mouse", "mousepad"], ["mouse", "mousepad"]]
    print("ok")
