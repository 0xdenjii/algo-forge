# LeetCode 43 - Multiply Strings
# Problem: Multiply two non-negative integers given as strings without
#   converting them to integers directly.
# Approach: Digit i*j contributes to positions i+j and i+j+1 of result.
# Time: O(n * m)  Space: O(n + m)


def multiply(a: str, b: str) -> str:
    if a == "0" or b == "0":
        return "0"
    res = [0] * (len(a) + len(b))
    for i in range(len(a) - 1, -1, -1):
        for j in range(len(b) - 1, -1, -1):
            total = (ord(a[i]) - 48) * (ord(b[j]) - 48) + res[i + j + 1]
            res[i + j + 1] = total % 10
            res[i + j] += total // 10
    return "".join(map(str, res)).lstrip("0")


if __name__ == "__main__":
    assert multiply("2", "3") == "6"
    assert multiply("123", "456") == "56088"
    assert multiply("999", "0") == "0"
    assert multiply("123456789", "987654321") == str(123456789 * 987654321)
    print("ok")
