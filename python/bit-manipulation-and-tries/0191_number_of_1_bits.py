# Problem: LeetCode 191 - Number of 1 Bits
#
# Return the number of set bits (Hamming weight) of an unsigned integer.
#
# Approach: Brian Kernighan: n &= n - 1 clears the lowest set bit each step.
# Complexity: O(popcount) time, O(1) space

def hamming_weight(n: int) -> int:
    count = 0
    while n:
        n &= n - 1
        count += 1
    return count


if __name__ == "__main__":
    assert hamming_weight(11) == 3
    assert hamming_weight(128) == 1
    assert hamming_weight(2**32 - 3) == 31
    print("ok")
