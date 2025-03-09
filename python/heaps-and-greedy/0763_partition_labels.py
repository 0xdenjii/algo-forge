"""LeetCode 763 - Partition Labels

Split s into as many parts as possible so each letter appears in at most one
part. Return the part sizes.

Approach: record each letter's last index; extend the current part's end to
cover every letter seen, and cut when i reaches that end.
Time: O(n)  Space: O(1) (alphabet)
"""


def partition_labels(s: str) -> list[int]:
    last = {ch: i for i, ch in enumerate(s)}
    sizes: list[int] = []
    start = end = 0
    for i, ch in enumerate(s):
        end = max(end, last[ch])
        if i == end:
            sizes.append(end - start + 1)
            start = i + 1
    return sizes


if __name__ == "__main__":
    assert partition_labels("ababcbacadefegdehijhklij") == [9, 7, 8]
    assert partition_labels("eccbbbbdec") == [10]
    print("ok")
