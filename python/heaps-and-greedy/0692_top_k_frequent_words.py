"""LeetCode 692 - Top K Frequent Words

Return the k most frequent words, sorted by frequency descending; ties are
broken by lexicographical order.

Approach: key each word by (-count, word); heapq.nsmallest yields the order.
Time: O(n log k)  Space: O(n)
"""
import heapq
from collections import Counter


def top_k_frequent(words: list[str], k: int) -> list[str]:
    counts = Counter(words)
    return heapq.nsmallest(k, counts, key=lambda w: (-counts[w], w))


if __name__ == "__main__":
    assert top_k_frequent(["i", "love", "leetcode", "i", "love", "coding"], 2) == ["i", "love"]
    assert top_k_frequent(
        ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], 4
    ) == ["the", "is", "sunny", "day"]
    print("ok")
