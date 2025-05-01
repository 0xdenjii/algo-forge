# Problem: LeetCode 338 - Counting Bits
#
# For every i in [0, n], return the number of 1 bits in i.
#
# Approach: DP: bits[i] = bits[i >> 1] + (i & 1).
# Complexity: O(n) time, O(n) space

def count_bits(n: int) -> list[int]:
    bits = [0] * (n + 1)
    for i in range(1, n + 1):
        bits[i] = bits[i >> 1] + (i & 1)
    return bits


if __name__ == "__main__":
    assert count_bits(5) == [0, 1, 1, 2, 1, 2]
    print("ok")
