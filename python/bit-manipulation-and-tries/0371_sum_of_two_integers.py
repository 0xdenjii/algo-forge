# Problem: LeetCode 371 - Sum of Two Integers
#
# Add two 32-bit signed integers without using + or -.
#
# Approach: XOR gives sum without carry; (a & b) << 1 gives carry. Emulate 32-bit
#   wrap-around with a mask, then convert back to signed.
# Complexity: O(32) time, O(1) space

MASK = 0xFFFFFFFF
MAX_INT = 0x7FFFFFFF


def get_sum(a: int, b: int) -> int:
    a &= MASK
    b &= MASK
    while b:
        a, b = (a ^ b) & MASK, ((a & b) << 1) & MASK
    return a if a <= MAX_INT else ~(a ^ MASK)


if __name__ == "__main__":
    assert get_sum(1, 2) == 3
    assert get_sum(-2, 3) == 1
    assert get_sum(-5, -7) == -12
    print("ok")
