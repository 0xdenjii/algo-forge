"""
Problem: LeetCode 42
Title: 42. Trapping Rain Water

Statement:
    Given an elevation map, compute how much water it can trap after raining.

Approach:
    The side with the smaller running max bounds the water there; process it
    and move inward.

Complexity: O(n) time, O(1) space.
"""

def trap(height: list[int]) -> int:
    lo, hi = 0, len(height) - 1
    lmax = rmax = water = 0
    while lo < hi:
        if height[lo] < height[hi]:
            lmax = max(lmax, height[lo])
            water += lmax - height[lo]
            lo += 1
        else:
            rmax = max(rmax, height[hi])
            water += rmax - height[hi]
            hi -= 1
    return water


if __name__ == "__main__":
    assert trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) == 6
    assert trap([4, 2, 0, 3, 2, 5]) == 9
    assert trap([]) == 0
    print("ok")
