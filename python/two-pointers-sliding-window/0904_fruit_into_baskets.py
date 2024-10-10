"""
Problem: LeetCode 904
Title: 904. Fruit Into Baskets

Statement:
    Walking a row of trees, pick from a contiguous segment holding at most two
    fruit types. Return the max number of fruits.

Approach:
    Variable window with a type-count map; shrink while more than 2 types.

Complexity: O(n) time, O(1) space.
"""

from collections import Counter


def total_fruit(fruits: list[int]) -> int:
    basket: Counter[int] = Counter()
    left = best = 0
    for right, f in enumerate(fruits):
        basket[f] += 1
        while len(basket) > 2:
            basket[fruits[left]] -= 1
            if basket[fruits[left]] == 0:
                del basket[fruits[left]]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    assert total_fruit([1, 2, 1]) == 3
    assert total_fruit([0, 1, 2, 2]) == 3
    assert total_fruit([1, 2, 3, 2, 2]) == 4
    print("ok")
