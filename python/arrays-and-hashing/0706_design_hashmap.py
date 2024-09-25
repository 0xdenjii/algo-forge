# Problem: LeetCode 706 - Design HashMap
#
# Implement a hash map (put, get, remove) without built-in hash tables.
# get returns -1 for missing keys.
#
# Approach: Separate chaining with [key, value] pairs and doubling resize.
# Complexity: O(1) average per operation, O(n) space

class MyHashMap:
    def __init__(self, capacity: int = 16) -> None:
        self._buckets: list[list[list[int]]] = [[] for _ in range(capacity)]
        self._size = 0

    def _bucket(self, key: int) -> list[list[int]]:
        return self._buckets[hash(key) % len(self._buckets)]

    def put(self, key: int, value: int) -> None:
        bucket = self._bucket(key)
        for pair in bucket:
            if pair[0] == key:
                pair[1] = value
                return
        bucket.append([key, value])
        self._size += 1
        if self._size > 0.75 * len(self._buckets):
            old = self._buckets
            self._buckets = [[] for _ in range(2 * len(old))]
            for b in old:
                for k, v in b:
                    self._bucket(k).append([k, v])

    def get(self, key: int) -> int:
        for k, v in self._bucket(key):
            if k == key:
                return v
        return -1

    def remove(self, key: int) -> None:
        bucket = self._bucket(key)
        for i, (k, _) in enumerate(bucket):
            if k == key:
                bucket.pop(i)
                self._size -= 1
                return


if __name__ == "__main__":
    m = MyHashMap()
    m.put(1, 1); m.put(2, 2)
    assert m.get(1) == 1 and m.get(3) == -1
    m.put(2, 1)
    assert m.get(2) == 1
    m.remove(2)
    assert m.get(2) == -1
    print("ok")
