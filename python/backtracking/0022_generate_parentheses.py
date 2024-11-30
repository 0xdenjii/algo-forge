"""
LeetCode 22 - Generate Parentheses
Problem: Given n pairs of parentheses, generate all combinations of
well-formed parentheses.
Approach: Place '(' while open < n, place ')' while close < open.
Time: O(4^n / sqrt(n)) (Catalan)  Space: O(n)
"""
from typing import List


def generate_parenthesis(n: int) -> List[str]:
    out: List[str] = []
    path: List[str] = []

    def dfs(open_: int, close: int) -> None:
        if len(path) == 2 * n:
            out.append("".join(path))
            return
        if open_ < n:
            path.append("(")
            dfs(open_ + 1, close)
            path.pop()
        if close < open_:
            path.append(")")
            dfs(open_, close + 1)
            path.pop()

    dfs(0, 0)
    return out


if __name__ == "__main__":
    assert generate_parenthesis(3) == ["((()))", "(()())", "(())()", "()(())", "()()()"]
    assert len(generate_parenthesis(5)) == 42
    print(generate_parenthesis(2))
