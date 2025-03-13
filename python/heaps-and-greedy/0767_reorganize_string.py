"""LeetCode 767 - Reorganize String

Rearrange s so that no two adjacent characters are equal; return "" if that
is impossible.

Approach: max-heap by remaining count; always emit the most frequent char
other than the previous one, holding the previous char back one step.
Time: O(n log a), a = alphabet size  Space: O(a)
"""
import heapq
from collections import Counter


def reorganize_string(s: str) -> str:
    heap = [(-c, ch) for ch, c in Counter(s).items()]
    heapq.heapify(heap)
    out: list[str] = []
    prev = (0, "")
    while heap:
        c, ch = heapq.heappop(heap)
        out.append(ch)
        if prev[0] < 0:
            heapq.heappush(heap, prev)
        prev = (c + 1, ch)
    return "".join(out) if len(out) == len(s) else ""


def is_valid(s: str) -> bool:
    return all(a != b for a, b in zip(s, s[1:]))


if __name__ == "__main__":
    r = reorganize_string("aab")
    assert sorted(r) == list("aab") and is_valid(r)
    assert reorganize_string("aaab") == ""
    r = reorganize_string("vvvlo")
    assert sorted(r) == sorted("vvvlo") and is_valid(r)
    print("ok")
