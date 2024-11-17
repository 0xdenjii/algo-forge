# LeetCode 127 - Word Ladder
# Given beginWord, endWord and a word list, return the number of words in the shortest
# sequence from beginWord to endWord changing one letter at a time (each in the list), or 0.
# Approach: BFS; precompute wildcard patterns ("h*t") -> words for fast neighbor lookup.
# Time: O(N * L^2)  Space: O(N * L^2)

from collections import defaultdict, deque


def ladder_length(begin: str, end: str, word_list: list[str]) -> int:
    words = set(word_list)
    if end not in words:
        return 0
    L = len(begin)
    buckets = defaultdict(list)
    for w in words | {begin}:
        for i in range(L):
            buckets[w[:i] + "*" + w[i + 1:]].append(w)
    q = deque([(begin, 1)])
    seen = {begin}
    while q:
        w, d = q.popleft()
        if w == end:
            return d
        for i in range(L):
            key = w[:i] + "*" + w[i + 1:]
            for nxt in buckets.pop(key, []):
                if nxt not in seen:
                    seen.add(nxt)
                    q.append((nxt, d + 1))
    return 0


if __name__ == "__main__":
    wl = ["hot", "dot", "dog", "lot", "log", "cog"]
    assert ladder_length("hit", "cog", wl) == 5
    assert ladder_length("hit", "cog", wl[:-1]) == 0
    print("ladder length:", ladder_length("hit", "cog", wl))
