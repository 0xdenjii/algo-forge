# LeetCode 752 - Open the Lock
# A 4-wheel lock starts at "0000"; each move turns one wheel by +/-1 (wrapping).
# Avoid deadends; return the minimum moves to reach target, or -1.
# Approach: BFS over the 10^4 states.
# Time: O(10^4 * 8)  Space: O(10^4)

from collections import deque


def open_lock(deadends: list[str], target: str) -> int:
    dead = set(deadends)
    if "0000" in dead:
        return -1
    q = deque([("0000", 0)])
    seen = {"0000"}
    while q:
        s, d = q.popleft()
        if s == target:
            return d
        for i in range(4):
            digit = int(s[i])
            for step in (1, -1):
                nxt = s[:i] + str((digit + step) % 10) + s[i + 1:]
                if nxt not in seen and nxt not in dead:
                    seen.add(nxt)
                    q.append((nxt, d + 1))
    return -1


if __name__ == "__main__":
    assert open_lock(["0201", "0101", "0102", "1212", "2002"], "0202") == 6
    assert open_lock(["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"], "8888") == -1
    print("moves:", open_lock(["0201", "0101", "0102", "1212", "2002"], "0202"))
