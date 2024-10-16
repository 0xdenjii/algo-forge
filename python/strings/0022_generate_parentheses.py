# LeetCode 22 - Generate Parentheses
# Problem: Generate all well-formed strings of n pairs of parentheses.
# Approach: Backtrack, adding "(" while open < n and ")" while close < open.
# Time: O(Catalan(n) * n)  Space: O(n) recursion


def generate_parenthesis(n: int) -> list[str]:
    out: list[str] = []

    def go(cur: list[str], open_: int, close: int) -> None:
        if len(cur) == 2 * n:
            out.append("".join(cur))
            return
        if open_ < n:
            cur.append("(")
            go(cur, open_ + 1, close)
            cur.pop()
        if close < open_:
            cur.append(")")
            go(cur, open_, close + 1)
            cur.pop()

    go([], 0, 0)
    return out


if __name__ == "__main__":
    assert generate_parenthesis(3) == ["((()))", "(()())", "(())()", "()(())", "()()()"]
    assert generate_parenthesis(1) == ["()"]
    print("ok")
