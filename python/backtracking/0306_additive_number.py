"""
LeetCode 306 - Additive Number
Problem: A string is additive if its digits form a sequence of at least three
numbers where each number after the first two is the sum of the previous two
(no leading zeros). Decide whether num is additive.
Approach: Backtrack over the choice of the first two numbers; the rest of the
sequence is then forced and verified greedily.
Time: O(n^3)  Space: O(n)
"""


def is_additive_number(num: str) -> bool:
    n = len(num)

    def valid(s: str) -> bool:
        return s == "0" or not s.startswith("0")

    for i in range(1, n):
        for j in range(i + 1, n):
            a, b = num[:i], num[i:j]
            if not (valid(a) and valid(b)):
                continue
            x, y, k = int(a), int(b), j
            while k < n:
                z = str(x + y)
                if not num.startswith(z, k):
                    break
                k += len(z)
                x, y = y, int(z)
            else:
                return True
    return False


if __name__ == "__main__":
    assert is_additive_number("112358")
    assert is_additive_number("199100199")
    assert not is_additive_number("1023")
    assert not is_additive_number("12")
    print("ok")
