# Problem: LeetCode 380 - Insert Delete GetRandom O(1)
#
# Design a set supporting insert, remove and getRandom (uniform) in average O(1).
#
# Approach: Array of values + dict value -> index; remove swaps the target with the last element.
# Complexity: O(1) average per operation, O(n) space

import random


class RandomizedSet:
    def __init__(self) -> None:
        self._vals: list[int] = []
        self._pos: dict[int, int] = {}

    def insert(self, val: int) -> bool:
        if val in self._pos:
            return False
        self._pos[val] = len(self._vals)
        self._vals.append(val)
        return True

    def remove(self, val: int) -> bool:
        i = self._pos.pop(val, None)
        if i is None:
            return False
        last = self._vals.pop()
        if i < len(self._vals):
            self._vals[i] = last
            self._pos[last] = i
        return True

    def get_random(self) -> int:
        return random.choice(self._vals)


if __name__ == "__main__":
    rs = RandomizedSet()
    assert rs.insert(1) and not rs.remove(2) and rs.insert(2)
    assert rs.get_random() in (1, 2)
    assert rs.remove(1) and not rs.insert(2)
    assert rs.get_random() == 2
    print("ok")
