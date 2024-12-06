"""
LeetCode 301 - Remove Invalid Parentheses
Problem: Remove the minimum number of parentheses so the string becomes
valid; return all unique results.
Approach: Count the minimum surplus '(' and ')' to remove, then DFS deciding
to keep or remove each paren while tracking the open balance; collect
results in a set.
Time: O(2^n * n)  Space: O(n)
"""
from typing import List, Set


def remove_invalid_parentheses(s: str) -> List[str]:
    rm_l = rm_r = 0
    for ch in s:
        if ch == "(":
            rm_l += 1
        elif ch == ")":
            if rm_l:
                rm_l -= 1
            else:
                rm_r += 1

    out: Set[str] = set()
    path: List[str] = []

    def dfs(i: int, open_: int, left: int, right: int) -> None:
        if i == len(s):
            if open_ == 0 and left == 0 and right == 0:
                out.add("".join(path))
            return
        ch = s[i]
        if ch == "(" and left > 0:
            dfs(i + 1, open_, left - 1, right)
        if ch == ")" and right > 0:
            dfs(i + 1, open_, left, right - 1)
        path.append(ch)
        if ch == "(":
            dfs(i + 1, open_ + 1, left, right)
        elif ch == ")":
            if open_ > 0:
                dfs(i + 1, open_ - 1, left, right)
        else:
            dfs(i + 1, open_, left, right)
        path.pop()

    dfs(0, 0, rm_l, rm_r)
    return sorted(out)


if __name__ == "__main__":
    assert remove_invalid_parentheses("()())()") == ["(())()", "()()()"]
    assert remove_invalid_parentheses("(a)())()") == ["(a())()", "(a)()()"]
    assert remove_invalid_parentheses(")(") == [""]
    print("ok")
