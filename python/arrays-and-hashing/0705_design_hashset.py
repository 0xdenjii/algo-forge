# Problem: LeetCode 705 - Design HashSet
#
# Implement a hash set (add, remove, contains) without built-in hash tables.
#
# Approach: Separate chaining over a list of buckets with resize at load factor 0.75.
# Complexity: O(1) average per operation, O(n) space

class MyHashSet:
    def __init__(self, capacity: int = 16) -> None:
        self._buckets: list[list[int]] = [[] for _ in range(capacity)]
        self._size = 0

    def _bucket(self, key: int) -> list[int]:
        return self._buckets[hash(key) % len(self._buckets)]

    def add(self, key: int) -> None:
        bucket = self._bucket(key)
        if key not in bucket:
            bucket.append(key)
            self._size += 1
            if self._size > 0.75 * len(self._buckets):
                self._resize()

    def remove(self, key: int) -> None:
        bucket = self._bucket(key)
        if key in bucket:
            bucket.remove(key)
            self._size -= 1

    def contains(self, key: int) -> bool:
        return key in self._bucket(key)

    def _resize(self) -> None:
        old = self._buckets
        self._buckets = [[] for _ in range(2 * len(old))]
        for bucket in old:
            for key in bucket:
                self._bucket(key).append(key)


if __name__ == "__main__":
    s = MyHashSet()
    s.add(1); s.add(2)
    assert s.contains(1) and not s.contains(3)
    s.add(2); s.remove(2)
    assert not s.contains(2)
    for i in range(1000):
        s.add(i)
    assert all(s.contains(i) for i in range(1000))
    print("ok")
