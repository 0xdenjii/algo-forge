# Problem: LeetCode 1442 - Count Triplets That Can Form Two Arrays of Equal XOR
#
# Count (i, j, k), i < j <= k, with arr[i..j-1] XOR == arr[j..k] XOR.
#
# Approach: Equivalent to prefix[i] == prefix[k+1]; each such pair contributes k - i choices of j.
#   Track count and index-sum per prefix value.
# Complexity: O(n) time, O(n) space

from collections import defaultdict


def count_triplets(arr: list[int]) -> int:
    count: defaultdict[int, int] = defaultdict(int)
    index_sum: defaultdict[int, int] = defaultdict(int)
    count[0] = 1
    prefix = total = 0
    for k, x in enumerate(arr):
        prefix ^= x
        # pairs (i, k) with prefix[i] == prefix[k+1] contribute k - i each
        total += count[prefix] * k - index_sum[prefix]
        count[prefix] += 1
        index_sum[prefix] += k + 1
    return total


if __name__ == "__main__":
    assert count_triplets([2, 3, 1, 6, 7]) == 4
    assert count_triplets([1, 1, 1, 1, 1]) == 10
    print("ok")
