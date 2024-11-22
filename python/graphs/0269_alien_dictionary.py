# LeetCode 269 - Alien Dictionary
# Given words sorted lexicographically in an unknown alphabet, return a valid letter order,
# or "" if the ordering is invalid.
# Approach: compare adjacent words to get edges at the first differing char; Kahn's toposort.
# Time: O(C) total characters  Space: O(U + min(U^2, N)) for U unique letters

from collections import deque


def alien_order(words: list[str]) -> str:
    adj = {c: set() for w in words for c in w}
    indeg = {c: 0 for c in adj}
    for a, b in zip(words, words[1:]):
        for x, y in zip(a, b):
            if x != y:
                if y not in adj[x]:
                    adj[x].add(y)
                    indeg[y] += 1
                break
        else:
            if len(a) > len(b):
                return ""
    q = deque(sorted(c for c in indeg if indeg[c] == 0))
    out = []
    while q:
        c = q.popleft()
        out.append(c)
        for d in sorted(adj[c]):
            indeg[d] -= 1
            if indeg[d] == 0:
                q.append(d)
    return "".join(out) if len(out) == len(indeg) else ""


if __name__ == "__main__":
    assert alien_order(["wrt", "wrf", "er", "ett", "rftt"]) == "wertf"
    assert alien_order(["z", "x", "z"]) == ""
    assert alien_order(["abc", "ab"]) == ""
    print("order:", alien_order(["wrt", "wrf", "er", "ett", "rftt"]))
