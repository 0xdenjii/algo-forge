"""
Problem: LeetCode 763
Title: 763. Partition Labels

Statement:
    Partition s into as many parts as possible so each letter appears in at most
    one part; return the part sizes.

Approach:
    Record last index per char; extend the current end, cut when i == end.

Complexity: O(n) time, O(alphabet) space.
"""

def partition_labels(s: str) -> list[int]:
    last = {ch: i for i, ch in enumerate(s)}
    res, start, end = [], 0, 0
    for i, ch in enumerate(s):
        end = max(end, last[ch])
        if i == end:
            res.append(end - start + 1)
            start = i + 1
    return res


if __name__ == "__main__":
    assert partition_labels("ababcbacadefegdehijhklij") == [9, 7, 8]
    assert partition_labels("eccbbbbdec") == [10]
    print("ok")
