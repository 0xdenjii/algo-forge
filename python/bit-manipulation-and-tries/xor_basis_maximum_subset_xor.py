# Problem: Maximum Subset XOR (linear XOR basis)
#
# Given non-negative integers, return the maximum XOR obtainable from any subset
# (classic competitive-programming XOR basis / Gaussian elimination over GF(2)).
#
# Approach: Maintain a basis indexed by highest bit; reduce each number into it, then greedily
#   build the maximum from high bits.
# Complexity: O(n * B) time, O(B) space with B = bit width

class XorBasis:
    def __init__(self, bits: int = 60) -> None:
        self.bits = bits
        self.basis = [0] * bits

    def insert(self, x: int) -> bool:
        for b in range(self.bits - 1, -1, -1):
            if not x >> b & 1:
                continue
            if not self.basis[b]:
                self.basis[b] = x
                return True
            x ^= self.basis[b]
        return False

    def max_xor(self) -> int:
        res = 0
        for b in range(self.bits - 1, -1, -1):
            res = max(res, res ^ self.basis[b])
        return res

    def can_make(self, x: int) -> bool:
        for b in range(self.bits - 1, -1, -1):
            if x >> b & 1 and self.basis[b]:
                x ^= self.basis[b]
        return x == 0


if __name__ == "__main__":
    xb = XorBasis()
    for v in (9, 8, 5):
        xb.insert(v)
    assert xb.max_xor() == 13
    assert xb.can_make(9 ^ 5) and not xb.can_make(2)
    print("ok")
