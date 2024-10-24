# Classic - Aho-Corasick Multi-Pattern Search
# Problem: Report every (index, pattern) occurrence of a set of patterns
#   in a text in one pass.
# Approach: Build a trie, add BFS failure links, merge output lists along
#   failure links, then stream the text through the automaton.
# Time: O(sum|patterns| + |text| + matches)  Space: O(sum|patterns|)
from collections import deque


class AhoCorasick:
    def __init__(self, patterns: list[str]) -> None:
        self.goto: list[dict[str, int]] = [{}]
        self.fail: list[int] = [0]
        self.out: list[list[str]] = [[]]
        for p in patterns:
            node = 0
            for ch in p:
                if ch not in self.goto[node]:
                    self.goto.append({})
                    self.fail.append(0)
                    self.out.append([])
                    self.goto[node][ch] = len(self.goto) - 1
                node = self.goto[node][ch]
            self.out[node].append(p)
        q = deque(self.goto[0].values())
        while q:
            u = q.popleft()
            for ch, v in self.goto[u].items():
                f = self.fail[u]
                while f and ch not in self.goto[f]:
                    f = self.fail[f]
                cand = self.goto[f].get(ch, 0)
                self.fail[v] = cand if cand != v else 0
                self.out[v] = self.out[v] + self.out[self.fail[v]]
                q.append(v)

    def search(self, text: str) -> list[tuple[int, str]]:
        res, node = [], 0
        for i, ch in enumerate(text):
            while node and ch not in self.goto[node]:
                node = self.fail[node]
            node = self.goto[node].get(ch, 0)
            for p in self.out[node]:
                res.append((i - len(p) + 1, p))
        return res


if __name__ == "__main__":
    ac = AhoCorasick(["he", "she", "his", "hers"])
    assert sorted(ac.search("ushers")) == [(1, "she"), (2, "he"), (2, "hers")]
    print("ok")
