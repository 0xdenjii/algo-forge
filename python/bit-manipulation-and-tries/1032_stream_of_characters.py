# Problem: LeetCode 1032 - Stream of Characters
#
# Given words, for each streamed letter report whether some suffix of the stream
# is one of the words.
#
# Approach: Trie of reversed words; on each query walk the recent stream backwards.
# Complexity: O(maxLen) per query, O(total chars) space

from collections import deque


class StreamChecker:
    def __init__(self, words: list[str]) -> None:
        self.root: dict = {}
        self.max_len = max(map(len, words))
        for w in words:
            node = self.root
            for c in reversed(w):
                node = node.setdefault(c, {})
            node["$"] = True
        self.stream: deque[str] = deque(maxlen=self.max_len)

    def query(self, letter: str) -> bool:
        self.stream.appendleft(letter)
        node = self.root
        for c in self.stream:
            node = node.get(c)
            if node is None:
                return False
            if "$" in node:
                return True
        return False


if __name__ == "__main__":
    sc = StreamChecker(["cd", "f", "kl"])
    got = [sc.query(c) for c in "abcdefghijkl"]
    assert got == [False, False, False, True, False, True, False, False, False, False, False, True]
    print("ok")
