# Problem: LeetCode 349 - Intersection of Two Arrays
#
# Return the unique elements present in both arrays.
#
# Approach: Set intersection, iterating over the smaller set.
# Complexity: O(n + m) time, O(min(n, m)) space

def intersection(a: list[int], b: list[int]) -> list[int]:
    sa, sb = set(a), set(b)
    if len(sa) > len(sb):
        sa, sb = sb, sa
    return [x for x in sa if x in sb]


if __name__ == "__main__":
    assert intersection([1, 2, 2, 1], [2, 2]) == [2]
    assert sorted(intersection([4, 9, 5], [9, 4, 9, 8, 4])) == [4, 9]
    print("ok")
