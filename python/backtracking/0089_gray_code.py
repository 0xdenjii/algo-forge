"""
LeetCode 89 - Gray Code
Problem: Return any n-bit Gray code sequence starting at 0: 2^n integers where
consecutive values (including last and first) differ in exactly one bit.
Approach: Backtracking over single-bit flips with a visited set; the
reflected formula i ^ (i >> 1) is used as a cross-check.
Time: O(n * 2^n)  Space: O(2^n)
"""
from typing import List


def gray_code(n: int) -> List[int]:
    total = 1 << n
    seen = {0}
    seq = [0]

    def dfs() -> bool:
        if len(seq) == total:
            return True
        cur = seq[-1]
        for b in range(n):
            nxt = cur ^ (1 << b)
            if nxt not in seen:
                seen.add(nxt)
                seq.append(nxt)
                if dfs():
                    return True
                seq.pop()
                seen.remove(nxt)
        return False

    dfs()
    return seq


def is_gray(seq: List[int]) -> bool:
    return all(bin(a ^ b).count("1") == 1 for a, b in zip(seq, seq[1:] + seq[:1]))


if __name__ == "__main__":
    for n in range(1, 7):
        s = gray_code(n)
        assert len(set(s)) == 1 << n and s[0] == 0 and is_gray(s)
    assert gray_code(2) == [0, 1, 3, 2] == [i ^ (i >> 1) for i in range(4)]
    print(gray_code(3))
