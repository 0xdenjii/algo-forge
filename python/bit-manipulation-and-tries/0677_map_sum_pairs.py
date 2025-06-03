# Problem: LeetCode 677 - Map Sum Pairs
#
# insert(key, val) sets/overwrites a key; sum(prefix) returns the total of
# values whose keys start with prefix.
#
# Approach: Trie nodes store prefix sums; on overwrite add the delta along the path.
# Complexity: O(L) per op, O(total chars) space

class MapSum:
    def __init__(self) -> None:
        self.values: dict[str, int] = {}
        self.root: dict = {"#": 0}

    def insert(self, key: str, val: int) -> None:
        delta = val - self.values.get(key, 0)
        self.values[key] = val
        node = self.root
        node["#"] += delta
        for c in key:
            node = node.setdefault(c, {"#": 0})
            node["#"] += delta

    def sum(self, prefix: str) -> int:
        node = self.root
        for c in prefix:
            if c not in node:
                return 0
            node = node[c]
        return node["#"]


if __name__ == "__main__":
    ms = MapSum()
    ms.insert("apple", 3)
    assert ms.sum("ap") == 3
    ms.insert("app", 2)
    assert ms.sum("ap") == 5
    ms.insert("apple", 1)
    assert ms.sum("ap") == 3 and ms.sum("b") == 0
    print("ok")
