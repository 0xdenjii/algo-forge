# Problem: LeetCode 347 - Top K Frequent Elements
#
# Return the k most frequent elements of nums (any order).
#
# Approach: Count, then bucket sort by frequency (bucket index = count).
# Complexity: O(n) time, O(n) space

from collections import Counter


def top_k_frequent(nums: list[int], k: int) -> list[int]:
    buckets: list[list[int]] = [[] for _ in range(len(nums) + 1)]
    for value, freq in Counter(nums).items():
        buckets[freq].append(value)
    result: list[int] = []
    for freq in range(len(buckets) - 1, 0, -1):
        for v in buckets[freq]:
            result.append(v)
            if len(result) == k:
                return result
    return result


if __name__ == "__main__":
    assert sorted(top_k_frequent([1, 1, 1, 2, 2, 3], 2)) == [1, 2]
    assert top_k_frequent([1], 1) == [1]
    print("ok")
