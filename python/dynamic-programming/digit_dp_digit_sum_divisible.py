# Classic - Digit DP: count integers in [lo, hi] whose digit sum is divisible by k
# Problem: Count integers x with lo <= x <= hi and (sum of digits of x) % k == 0.
#   Ranges may be huge (up to 10^18), so brute force is impossible.
# Approach: f(N) counts 0..N via memoised recursion over (position, sum mod k,
#   tight flag); answer = f(hi) - f(lo - 1).
# Time: O(digits * k * 10)  Space: O(digits * k)
from functools import lru_cache


def count_upto(n: int, k: int) -> int:
    if n < 0:
        return 0
    digits = list(map(int, str(n)))

    @lru_cache(maxsize=None)
    def go(pos: int, rem: int, tight: bool) -> int:
        if pos == len(digits):
            return int(rem == 0)
        limit = digits[pos] if tight else 9
        return sum(go(pos + 1, (rem + d) % k, tight and d == limit) for d in range(limit + 1))

    return go(0, 0, True)


def count_range(lo: int, hi: int, k: int) -> int:
    return count_upto(hi, k) - count_upto(lo - 1, k)


if __name__ == "__main__":
    brute = sum(1 for x in range(1, 1001) if sum(map(int, str(x))) % 7 == 0)
    assert count_range(1, 1000, 7) == brute
    print("ok", count_range(1, 10**18, 9))
