# Problem: LeetCode 342 - Power of Four
#
# Return True if n is a power of four.
#
# Approach: Power of two whose single bit sits at an even position (mask 0x55555555).
# Complexity: O(1) time, O(1) space

def is_power_of_four(n: int) -> bool:
    return n > 0 and n & (n - 1) == 0 and n & 0x55555555 != 0


if __name__ == "__main__":
    assert is_power_of_four(16) and is_power_of_four(1)
    assert not is_power_of_four(8) and not is_power_of_four(5)
    print("ok")
