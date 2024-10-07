"""
Problem: LeetCode 11
Title: 11. Container With Most Water

Statement:
    Given heights of n vertical lines, find two lines that with the x-axis
    form a container holding the most water. Return the max area.

Approach:
    Start at both ends; the shorter wall limits area, so moving it is the only
    way to possibly improve.

Complexity: O(n) time, O(1) space.
"""

def max_area(height: list[int]) -> int:
    lo, hi, best = 0, len(height) - 1, 0
    while lo < hi:
        best = max(best, (hi - lo) * min(height[lo], height[hi]))
        if height[lo] < height[hi]:
            lo += 1
        else:
            hi -= 1
    return best


if __name__ == "__main__":
    assert max_area([1, 8, 6, 2, 5, 4, 8, 3, 7]) == 49
    assert max_area([1, 1]) == 1
    print("ok")
