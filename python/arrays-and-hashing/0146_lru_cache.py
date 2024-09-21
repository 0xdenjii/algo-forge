# Problem: LeetCode 146 - LRU Cache
#
# Design an LRU cache with get(key) and put(key, value) in O(1); evict the
# least recently used key when capacity is exceeded. get returns -1 if absent.
#
# Approach: Hash map with insertion order (OrderedDict); move_to_end on access, popitem(last=False) to evict.
# Complexity: O(1) per operation, O(capacity) space

from collections import OrderedDict


class LRUCache:
    def __init__(self, capacity: int) -> None:
        self.capacity = capacity
        self._data: OrderedDict[int, int] = OrderedDict()

    def get(self, key: int) -> int:
        if key not in self._data:
            return -1
        self._data.move_to_end(key)
        return self._data[key]

    def put(self, key: int, value: int) -> None:
        self._data[key] = value
        self._data.move_to_end(key)
        if len(self._data) > self.capacity:
            self._data.popitem(last=False)


if __name__ == "__main__":
    c = LRUCache(2)
    c.put(1, 1); c.put(2, 2)
    assert c.get(1) == 1
    c.put(3, 3)
    assert c.get(2) == -1
    c.put(4, 4)
    assert c.get(1) == -1 and c.get(3) == 3 and c.get(4) == 4
    print("ok")
