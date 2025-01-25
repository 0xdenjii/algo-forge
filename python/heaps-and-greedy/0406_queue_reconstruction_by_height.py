"""LeetCode 406 - Queue Reconstruction by Height

people[i] = [h, k]: person of height h with exactly k people of height >= h
in front. Reconstruct the queue.

Approach: sort by height desc, k asc; insert each person at index k. Shorter
people inserted later never affect taller people's counts.
Time: O(n^2) (list inserts)  Space: O(n)
"""


def reconstruct_queue(people: list[list[int]]) -> list[list[int]]:
    queue: list[list[int]] = []
    for p in sorted(people, key=lambda p: (-p[0], p[1])):
        queue.insert(p[1], p)
    return queue


if __name__ == "__main__":
    assert reconstruct_queue([[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]]) == [
        [5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]
    ]
    print("ok")
