# Problem: LeetCode 350 - Intersection of Two Arrays II
#
# Return the intersection with multiplicity: each element appears as many
# times as it shows in both arrays.
#
# Approach: Counter intersection (min of counts).
# Complexity: O(n + m) time, O(min(n, m)) space

from collections import Counter


def intersect(a: list[int], b: list[int]) -> list[int]:
    return list((Counter(a) & Counter(b)).elements())


if __name__ == "__main__":
    assert intersect([1, 2, 2, 1], [2, 2]) == [2, 2]
    assert sorted(intersect([4, 9, 5], [9, 4, 9, 8, 4])) == [4, 9]
    print("ok")
