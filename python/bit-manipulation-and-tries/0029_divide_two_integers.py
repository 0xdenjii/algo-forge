# Problem: LeetCode 29 - Divide Two Integers
#
# Divide two 32-bit integers without *, /, or %, truncating toward zero;
# clamp overflow to INT_MAX.
#
# Approach: Subtract the largest shifted divisor (divisor << k) repeatedly, accumulating 1 << k.
# Complexity: O(log^2 n) time, O(1) space

INT_MAX, INT_MIN = 2**31 - 1, -(2**31)


def divide(dividend: int, divisor: int) -> int:
    if dividend == INT_MIN and divisor == -1:
        return INT_MAX
    negative = (dividend < 0) != (divisor < 0)
    a, b = abs(dividend), abs(divisor)
    quotient = 0
    while a >= b:
        shift = 0
        while a >= b << (shift + 1):
            shift += 1
        a -= b << shift
        quotient += 1 << shift
    return -quotient if negative else quotient


if __name__ == "__main__":
    assert divide(10, 3) == 3
    assert divide(7, -3) == -2
    assert divide(INT_MIN, -1) == INT_MAX
    assert divide(INT_MIN, 1) == INT_MIN
    print("ok")
