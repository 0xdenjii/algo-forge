# Problem: LeetCode 421 - Maximum XOR of Two Numbers in an Array
#
# Return the maximum nums[i] ^ nums[j] over all pairs.
#
# Approach: Binary trie of 31-bit numbers; for each number greedily walk toward the opposite bit.
# Complexity: O(31 n) time, O(31 n) space

HIGH_BIT = 30


class BitTrie:
    def __init__(self) -> None:
        self.children: list[list[int]] = [[0, 0]]

    def insert(self, x: int) -> None:
        node = 0
        for b in range(HIGH_BIT, -1, -1):
            bit = x >> b & 1
            if not self.children[node][bit]:
                self.children.append([0, 0])
                self.children[node][bit] = len(self.children) - 1
            node = self.children[node][bit]

    def max_xor(self, x: int) -> int:
        node, res = 0, 0
        for b in range(HIGH_BIT, -1, -1):
            want = (x >> b & 1) ^ 1
            if self.children[node][want]:
                res |= 1 << b
                node = self.children[node][want]
            else:
                node = self.children[node][want ^ 1]
        return res


def find_maximum_xor(nums: list[int]) -> int:
    trie = BitTrie()
    best = 0
    for x in nums:
        trie.insert(x)
        best = max(best, trie.max_xor(x))
    return best


if __name__ == "__main__":
    assert find_maximum_xor([3, 10, 5, 25, 2, 8]) == 28
    assert find_maximum_xor([14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]) == 127
    print("ok")
