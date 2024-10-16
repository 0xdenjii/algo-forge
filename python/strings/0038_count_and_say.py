# LeetCode 38 - Count and Say
# Problem: countAndSay(1) = "1"; each next term is the run-length
#   description of the previous term. Return the n-th term.
# Approach: Iterate n-1 times using itertools.groupby for runs.
# Time: O(total length of terms)  Space: O(length of term)
from itertools import groupby


def count_and_say(n: int) -> str:
    s = "1"
    for _ in range(n - 1):
        s = "".join(f"{len(list(g))}{d}" for d, g in groupby(s))
    return s


if __name__ == "__main__":
    assert [count_and_say(i) for i in range(1, 6)] == ["1", "11", "21", "1211", "111221"]
    print("ok")
