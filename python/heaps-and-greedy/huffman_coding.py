"""Huffman Coding (classic greedy)

Given symbol frequencies, build a prefix-free binary code minimising the
total encoded length sum(freq * code_length).

Approach: repeatedly merge the two least-frequent trees with a min-heap; a
counter breaks ties so subtrees are never compared directly.
Time: O(n log n)  Space: O(n)
"""
from __future__ import annotations

import heapq
import itertools
from collections import Counter


def huffman_codes(freq: dict[str, int]) -> dict[str, str]:
    if len(freq) == 1:
        return {next(iter(freq)): "0"}
    tie = itertools.count()
    heap: list = [(f, next(tie), sym) for sym, f in freq.items()]
    heapq.heapify(heap)
    while len(heap) > 1:
        f1, _, a = heapq.heappop(heap)
        f2, _, b = heapq.heappop(heap)
        heapq.heappush(heap, (f1 + f2, next(tie), (a, b)))
    codes: dict[str, str] = {}

    def walk(node, prefix: str) -> None:
        if isinstance(node, str):
            codes[node] = prefix
        else:
            walk(node[0], prefix + "0")
            walk(node[1], prefix + "1")

    walk(heap[0][2], "")
    return codes


def encode(text: str) -> tuple[str, dict[str, str]]:
    codes = huffman_codes(Counter(text))
    return "".join(codes[c] for c in text), codes


def decode(bits: str, codes: dict[str, str]) -> str:
    rev = {v: k for k, v in codes.items()}
    out, cur = [], ""
    for b in bits:
        cur += b
        if cur in rev:
            out.append(rev[cur])
            cur = ""
    return "".join(out)


if __name__ == "__main__":
    text = "abracadabra alakazam"
    bits, codes = encode(text)
    assert decode(bits, codes) == text
    freq = {"a": 45, "b": 13, "c": 12, "d": 16, "e": 9, "f": 5}
    c = huffman_codes(freq)
    assert sum(freq[s] * len(c[s]) for s in freq) == 224  # CLRS example
    print("ok", len(bits), "bits")
