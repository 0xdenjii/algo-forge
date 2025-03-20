# Problem: LeetCode 190 - Reverse Bits
#
# Reverse the bits of a 32-bit unsigned integer.
#
# Approach: Divide and conquer swaps: halves, bytes, nibbles, pairs, bits.
# Complexity: O(1) time, O(1) space

def reverse_bits(n: int) -> int:
    n = ((n >> 16) | (n << 16)) & 0xFFFFFFFF
    n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8)
    n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4)
    n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2)
    n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1)
    return n


if __name__ == "__main__":
    assert reverse_bits(0b00000010100101000001111010011100) == 964176192
    assert reverse_bits(0xFFFFFFFD) == 3221225471
    print("ok")
