# Problem: LeetCode 51 - N-Queens (bitmask backtracking)
#
# Construct all placements of n non-attacking queens on an n x n board.
#
# Approach: Backtrack row by row with three bitmasks (columns, diagonals, anti-diagonals);
#   free = ~(cols | d1 | d2) & full, iterate lowest set bits.
# Complexity: O(n!) time, O(n) space

def solve_n_queens(n: int) -> list[list[str]]:
    full = (1 << n) - 1
    res: list[list[str]] = []
    placement: list[int] = []

    def place(cols: int, d1: int, d2: int) -> None:
        if cols == full:
            res.append(["." * c + "Q" + "." * (n - c - 1) for c in placement])
            return
        free = ~(cols | d1 | d2) & full
        while free:
            bit = free & -free
            free ^= bit
            placement.append(bit.bit_length() - 1)
            place(cols | bit, (d1 | bit) << 1 & full, (d2 | bit) >> 1)
            placement.pop()

    place(0, 0, 0)
    return res


if __name__ == "__main__":
    assert [len(solve_n_queens(n)) for n in range(1, 9)] == [1, 0, 0, 2, 10, 4, 40, 92]
    print("ok")
