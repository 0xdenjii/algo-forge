# LeetCode 91 - Decode Ways
# Problem: Letters A-Z map to "1".."26". Given a digit string, count the ways to
#   decode it (leading zeros in a chunk are invalid).
# Approach: ways(i) = ways(i-1) if s[i-1] != '0' + ways(i-2) if s[i-2:i] in 10..26.
# Time: O(n)  Space: O(1)


def num_decodings(s: str) -> int:
    prev2, prev1 = 0, 1  # ways(i-2), ways(i-1); ways(0) = 1
    for i in range(1, len(s) + 1):
        cur = prev1 if s[i - 1] != "0" else 0
        if i >= 2 and 10 <= int(s[i - 2:i]) <= 26:
            cur += prev2
        prev2, prev1 = prev1, cur
    return prev1 if s else 0


if __name__ == "__main__":
    assert num_decodings("12") == 2
    assert num_decodings("226") == 3
    assert num_decodings("06") == 0
    assert num_decodings("10") == 1
    print("ok")
