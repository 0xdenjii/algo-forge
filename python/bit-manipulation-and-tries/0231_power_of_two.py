# Problem: LeetCode 231 - Power of Two
#
# Return True if n is a power of two.
#
# Approach: A power of two has exactly one set bit: n > 0 and n & (n - 1) == 0.
# Complexity: O(1) time, O(1) space

def is_power_of_two(n: int) -> bool:
    return n > 0 and n & (n - 1) == 0


if __name__ == "__main__":
    assert is_power_of_two(1) and is_power_of_two(1024)
    assert not is_power_of_two(0) and not is_power_of_two(6) and not is_power_of_two(-8)
    print("ok")
