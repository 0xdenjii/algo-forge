# LeetCode 8 - String to Integer (atoi)
# Problem: Parse leading whitespace, optional sign, then digits into a
#   32-bit signed int, clamping on overflow; stop at first non-digit.
# Approach: Single left-to-right scan with explicit clamping.
# Time: O(n)  Space: O(1)

INT_MAX, INT_MIN = 2**31 - 1, -(2**31)


def my_atoi(s: str) -> int:
    i, n = 0, len(s)
    while i < n and s[i] == " ":
        i += 1
    sign = 1
    if i < n and s[i] in "+-":
        sign = -1 if s[i] == "-" else 1
        i += 1
    val = 0
    while i < n and "0" <= s[i] <= "9":
        val = val * 10 + (ord(s[i]) - 48)
        if val > INT_MAX:
            return INT_MAX if sign == 1 else INT_MIN
        i += 1
    return sign * val


if __name__ == "__main__":
    assert my_atoi("42") == 42
    assert my_atoi("   -042") == -42
    assert my_atoi("1337c0d3") == 1337
    assert my_atoi("words 987") == 0
    assert my_atoi("-91283472332") == INT_MIN
    assert my_atoi("2147483648") == INT_MAX
    print("ok")
