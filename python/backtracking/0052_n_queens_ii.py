"""
LeetCode 52 - N-Queens II
Problem: Return the number of distinct solutions to the n-queens puzzle.
Approach: Bitmask backtracking; free positions in a row are
~(cols | diag | anti) masked to n bits; the lowest set bit is placed each step
and diagonal masks shift by one per row.
Time: O(n!)  Space: O(n)
"""


def total_n_queens(n: int) -> int:
    full = (1 << n) - 1

    def dfs(cols: int, diag: int, anti: int) -> int:
        if cols == full:
            return 1
        count = 0
        free = full & ~(cols | diag | anti)
        while free:
            bit = free & -free
            free ^= bit
            count += dfs(cols | bit, ((diag | bit) << 1) & full, (anti | bit) >> 1)
        return count

    return dfs(0, 0, 0)


if __name__ == "__main__":
    expected = {1: 1, 2: 0, 3: 0, 4: 2, 5: 10, 6: 4, 7: 40, 8: 92, 9: 352, 10: 724}
    for n, v in expected.items():
        assert total_n_queens(n) == v
    print(total_n_queens(8))
