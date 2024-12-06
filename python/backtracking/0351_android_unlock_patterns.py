"""
LeetCode 351 - Android Unlock Patterns
Problem: Count unlock patterns on a 3x3 keypad with length between m and n.
A move may pass over a key only if that key was already used.
Approach: 'skip' table for jumps; DFS from keys 1, 2, 5 and use symmetry
(corners x4, edges x4, center x1).
Time: O(9!)  Space: O(9)
"""


def number_of_patterns(m: int, n: int) -> int:
    skip = [[0] * 10 for _ in range(10)]
    for a, b, mid in ((1, 3, 2), (1, 7, 4), (3, 9, 6), (7, 9, 8),
                      (1, 9, 5), (3, 7, 5), (2, 8, 5), (4, 6, 5)):
        skip[a][b] = skip[b][a] = mid
    used = [False] * 10

    def dfs(cur: int, length: int) -> int:
        count = 1 if length >= m else 0
        if length == n:
            return count
        used[cur] = True
        for nxt in range(1, 10):
            mid = skip[cur][nxt]
            if not used[nxt] and (mid == 0 or used[mid]):
                count += dfs(nxt, length + 1)
        used[cur] = False
        return count

    return dfs(1, 1) * 4 + dfs(2, 1) * 4 + dfs(5, 1)


if __name__ == "__main__":
    assert number_of_patterns(1, 1) == 9
    assert number_of_patterns(1, 2) == 65
    assert number_of_patterns(4, 9) == 389112
    print("ok")
