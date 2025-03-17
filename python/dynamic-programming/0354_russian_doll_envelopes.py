# LeetCode 354 - Russian Doll Envelopes
# Problem: Envelopes (w, h) nest if both dimensions are strictly smaller. Return
#   the maximum number of envelopes that can be nested.
# Approach: Sort by width asc and height desc (so equal widths cannot chain),
#   then strict LIS on heights with binary search.
# Time: O(n log n)  Space: O(n)
from bisect import bisect_left


def max_envelopes(envelopes: list[list[int]]) -> int:
    envelopes = sorted(envelopes, key=lambda e: (e[0], -e[1]))
    tails: list[int] = []
    for _, h in envelopes:
        i = bisect_left(tails, h)
        if i == len(tails):
            tails.append(h)
        else:
            tails[i] = h
    return len(tails)


if __name__ == "__main__":
    assert max_envelopes([[5, 4], [6, 4], [6, 7], [2, 3]]) == 3
    assert max_envelopes([[1, 1], [1, 1], [1, 1]]) == 1
    print("ok")
