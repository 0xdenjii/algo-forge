# LeetCode 70 - Climbing Stairs
# Problem: You climb a staircase of n steps, taking 1 or 2 steps at a time.
#   Count the distinct ways to reach the top.
# Approach: ways(i) = ways(i-1) + ways(i-2); keep only the last two values (Fibonacci).
# Time: O(n)  Space: O(1)


def climb_stairs(n: int) -> int:
    a, b = 1, 1
    for _ in range(n):
        a, b = b, a + b
    return a


if __name__ == "__main__":
    assert climb_stairs(1) == 1
    assert climb_stairs(2) == 2
    assert climb_stairs(5) == 8
    print("ok", climb_stairs(45))
