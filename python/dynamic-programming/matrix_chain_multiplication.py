# Classic (CLRS 15.2) - Matrix Chain Multiplication
# Problem: Matrices A1..An have dimensions dims[i-1] x dims[i]. Find the
#   parenthesisation minimising scalar multiplications and print it.
# Approach: Interval DP: cost[i][j] = min over k of cost[i][k] + cost[k+1][j]
#   + dims[i-1]*dims[k]*dims[j]; record split points.
# Time: O(n^3)  Space: O(n^2)
from math import inf


def matrix_chain_order(dims: list[int]) -> tuple[int, str]:
    n = len(dims) - 1
    cost = [[0] * (n + 1) for _ in range(n + 1)]
    split = [[0] * (n + 1) for _ in range(n + 1)]
    for length in range(2, n + 1):
        for i in range(1, n - length + 2):
            j = i + length - 1
            cost[i][j] = inf
            for k in range(i, j):
                c = cost[i][k] + cost[k + 1][j] + dims[i - 1] * dims[k] * dims[j]
                if c < cost[i][j]:
                    cost[i][j], split[i][j] = c, k

    def paren(i: int, j: int) -> str:
        if i == j:
            return f"A{i}"
        k = split[i][j]
        return f"({paren(i, k)}{paren(k + 1, j)})"

    return int(cost[1][n]), paren(1, n)


if __name__ == "__main__":
    c, p = matrix_chain_order([30, 35, 15, 5, 10, 20, 25])
    assert c == 15125 and p == "((A1(A2A3))((A4A5)A6))"
    assert matrix_chain_order([10, 20])[0] == 0
    print("ok", c, p)
