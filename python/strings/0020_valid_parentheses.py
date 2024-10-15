# LeetCode 20 - Valid Parentheses
# Problem: Given a string of ()[]{} decide whether brackets are closed
#   in the correct order.
# Approach: Push expected closers on a stack; pop and compare.
# Time: O(n)  Space: O(n)

PAIRS = {"(": ")", "[": "]", "{": "}"}


def is_valid(s: str) -> bool:
    stack: list[str] = []
    for ch in s:
        if ch in PAIRS:
            stack.append(PAIRS[ch])
        elif not stack or stack.pop() != ch:
            return False
    return not stack


if __name__ == "__main__":
    assert is_valid("()[]{}")
    assert is_valid("{[]}")
    assert not is_valid("(]")
    assert not is_valid("([)]")
    assert not is_valid("(")
    print("ok")
