"""LeetCode 135 - Candy

Children in a line have ratings. Each gets at least one candy, and a child
with a higher rating than a neighbour gets more than that neighbour.
Return the minimum total candies.

Approach: two greedy passes - left-to-right enforces the left neighbour rule,
right-to-left the right neighbour rule, taking the max of both.
Time: O(n)  Space: O(n)
"""


def candy(ratings: list[int]) -> int:
    n = len(ratings)
    c = [1] * n
    for i in range(1, n):
        if ratings[i] > ratings[i - 1]:
            c[i] = c[i - 1] + 1
    for i in range(n - 2, -1, -1):
        if ratings[i] > ratings[i + 1]:
            c[i] = max(c[i], c[i + 1] + 1)
    return sum(c)


if __name__ == "__main__":
    assert candy([1, 0, 2]) == 5
    assert candy([1, 2, 2]) == 4
    assert candy([1, 3, 4, 5, 2]) == 11
    print("ok")
