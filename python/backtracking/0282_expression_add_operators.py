"""
LeetCode 282 - Expression Add Operators
Problem: Given a digit string and a target, insert '+', '-', '*' between
digits so the expression evaluates to target. Operands may not have leading
zeros. Return all such expressions.
Approach: DFS tracking the current value and the last operand so that a
multiplication can undo the previous term: value - last + last * cur.
Time: O(4^n * n)  Space: O(n)
"""
from typing import List


def add_operators(num: str, target: int) -> List[str]:
    out: List[str] = []

    def dfs(i: int, expr: str, value: int, last: int) -> None:
        if i == len(num):
            if value == target:
                out.append(expr)
            return
        for j in range(i + 1, len(num) + 1):
            s = num[i:j]
            if len(s) > 1 and s[0] == "0":
                break
            cur = int(s)
            if i == 0:
                dfs(j, s, cur, cur)
            else:
                dfs(j, f"{expr}+{s}", value + cur, cur)
                dfs(j, f"{expr}-{s}", value - cur, -cur)
                dfs(j, f"{expr}*{s}", value - last + last * cur, last * cur)

    dfs(0, "", 0, 0)
    return out


if __name__ == "__main__":
    assert sorted(add_operators("123", 6)) == ["1*2*3", "1+2+3"]
    assert sorted(add_operators("232", 8)) == ["2*3+2", "2+3*2"]
    assert sorted(add_operators("105", 5)) == ["1*0+5", "10-5"]
    assert add_operators("3456237490", 9191) == []
    print("ok")
