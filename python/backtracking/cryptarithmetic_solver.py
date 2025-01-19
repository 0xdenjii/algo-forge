"""
Cryptarithmetic Puzzle Solver (classic; cf. LeetCode 1307 Verbal Arithmetic)
Problem: Assign distinct digits to letters so that the sum of the operand
words equals the result word (e.g. SEND + MORE = MONEY); leading letters of
multi-letter words may not be zero. Return one assignment or None.
Approach: Reduce to a linear equation: each letter gets a weight (place values
on the left minus place values on the right). Assign digits to letters with
the largest |weight| first, pruning when the remaining letters cannot bring
the running total back to 0.
Time: O(10! / (10 - L)!) worst  Space: O(L)
"""
from typing import Dict, List, Optional


def solve(words: List[str], result: str) -> Optional[Dict[str, int]]:
    weight: Dict[str, int] = {}
    for w in words:
        for i, ch in enumerate(reversed(w)):
            weight[ch] = weight.get(ch, 0) + 10 ** i
    for i, ch in enumerate(reversed(result)):
        weight[ch] = weight.get(ch, 0) - 10 ** i
    leading = {w[0] for w in words + [result] if len(w) > 1}
    letters = sorted(weight, key=lambda c: -abs(weight[c]))
    if len(letters) > 10:
        return None
    reach = [0] * (len(letters) + 1)  # max |contribution| of letters[i:]
    for i in range(len(letters) - 1, -1, -1):
        reach[i] = reach[i + 1] + 9 * abs(weight[letters[i]])
    assign: Dict[str, int] = {}
    used = [False] * 10

    def dfs(i: int, total: int) -> bool:
        if i == len(letters):
            return total == 0
        if abs(total) > reach[i]:
            return False
        ch = letters[i]
        for d in range(1 if ch in leading else 0, 10):
            if not used[d]:
                used[d] = True
                assign[ch] = d
                if dfs(i + 1, total + d * weight[ch]):
                    return True
                used[d] = False
        assign.pop(ch, None)
        return False

    return dict(assign) if dfs(0, 0) else None


def value(word: str, mapping: Dict[str, int]) -> int:
    return int("".join(str(mapping[c]) for c in word))


if __name__ == "__main__":
    sol = solve(["SEND", "MORE"], "MONEY")
    assert sol is not None
    assert value("SEND", sol) + value("MORE", sol) == value("MONEY", sol) == 10652
    assert solve(["LEET", "CODE"], "POINT") is None
    print(sol)
