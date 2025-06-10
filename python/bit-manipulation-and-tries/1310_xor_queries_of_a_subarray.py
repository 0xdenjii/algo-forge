# Problem: LeetCode 1310 - XOR Queries of a Subarray
#
# For each query [l, r], return arr[l] ^ ... ^ arr[r].
#
# Approach: Prefix XOR: px[r+1] ^ px[l].
# Complexity: O(n + q) time, O(n) space

from itertools import accumulate
from operator import xor


def xor_queries(arr: list[int], queries: list[list[int]]) -> list[int]:
    px = [0, *accumulate(arr, xor)]
    return [px[r + 1] ^ px[l] for l, r in queries]


if __name__ == "__main__":
    assert xor_queries([1, 3, 4, 8], [[0, 1], [1, 2], [0, 3], [3, 3]]) == [2, 7, 14, 8]
    print("ok")
