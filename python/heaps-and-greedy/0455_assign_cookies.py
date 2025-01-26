"""LeetCode 455 - Assign Cookies

Child i is content with a cookie of size >= g[i]; each child gets at most
one cookie. Return the maximum number of content children.

Approach: sort both; two pointers give each cookie to the least greedy
child it can satisfy.
Time: O(n log n + m log m)  Space: O(1) extra
"""


def find_content_children(g: list[int], s: list[int]) -> int:
    g, s = sorted(g), sorted(s)
    child = 0
    for cookie in s:
        if child < len(g) and cookie >= g[child]:
            child += 1
    return child


if __name__ == "__main__":
    assert find_content_children([1, 2, 3], [1, 1]) == 1
    assert find_content_children([1, 2], [1, 2, 3]) == 2
    print("ok")
