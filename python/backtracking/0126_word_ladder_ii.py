"""
LeetCode 126 - Word Ladder II
Problem: Return all shortest transformation sequences from beginWord to
endWord, changing one letter at a time, every intermediate word in wordList.
Approach: BFS layer by layer to build a parent map holding only shortest
edges, then backtrack from endWord to beginWord to enumerate the paths.
Time: O(N * L * 26 + paths * L)  Space: O(N * L)
"""
from collections import defaultdict
from typing import Dict, List, Set


def find_ladders(begin: str, end: str, word_list: List[str]) -> List[List[str]]:
    words = set(word_list)
    if end not in words:
        return []
    parents: Dict[str, Set[str]] = defaultdict(set)
    layer = {begin}
    words.discard(begin)
    found = False
    while layer and not found:
        nxt: Dict[str, Set[str]] = defaultdict(set)
        for w in layer:
            for i in range(len(w)):
                for ch in "abcdefghijklmnopqrstuvwxyz":
                    cand = w[:i] + ch + w[i + 1:]
                    if cand in words:
                        nxt[cand].add(w)
        words -= nxt.keys()
        for child, ps in nxt.items():
            parents[child] |= ps
        found = end in nxt
        layer = set(nxt)
    if not found:
        return []

    out: List[List[str]] = []
    path = [end]

    def backtrack(w: str) -> None:
        if w == begin:
            out.append(path[::-1])
            return
        for p in sorted(parents[w]):
            path.append(p)
            backtrack(p)
            path.pop()

    backtrack(end)
    return out


if __name__ == "__main__":
    res = find_ladders("hit", "cog", ["hot", "dot", "dog", "lot", "log", "cog"])
    assert sorted(res) == [["hit", "hot", "dot", "dog", "cog"], ["hit", "hot", "lot", "log", "cog"]]
    assert find_ladders("hit", "cog", ["hot", "dot", "dog", "lot", "log"]) == []
    print(res)
