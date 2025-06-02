# Problem: LeetCode 461 - Hamming Distance
#
# Return the number of positions at which the bits of x and y differ.
#
# Approach: popcount(x ^ y).
# Complexity: O(1) time, O(1) space

def hamming_distance(x: int, y: int) -> int:
    return bin(x ^ y).count("1")


if __name__ == "__main__":
    assert hamming_distance(1, 4) == 2
    assert hamming_distance(3, 1) == 1
    print("ok")
