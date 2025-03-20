# Problem: LeetCode 89 - Gray Code
#
# Return an n-bit Gray code sequence starting at 0 where adjacent values
# (including first and last) differ in exactly one bit.
#
# Approach: The i-th Gray code is i ^ (i >> 1).
# Complexity: O(2^n) time, O(2^n) output

def gray_code(n: int) -> list[int]:
    return [i ^ (i >> 1) for i in range(1 << n)]


if __name__ == "__main__":
    seq = gray_code(3)
    assert len(set(seq)) == 8 and seq[0] == 0
    for a, b in zip(seq, seq[1:] + seq[:1]):
        assert bin(a ^ b).count("1") == 1
    print("ok")
