# LeetCode 394 - Decode String
# Problem: Decode k[encoded] patterns (possibly nested), e.g.
#   "3[a2[c]]" -> "accaccacc".
# Approach: Stack of (string-so-far, multiplier) on "[" and pop on "]".
# Time: O(output length)  Space: O(output length)


def decode_string(s: str) -> str:
    stack: list[tuple[list[str], int]] = []
    cur: list[str] = []
    num = 0
    for ch in s:
        if ch.isdigit():
            num = num * 10 + int(ch)
        elif ch == "[":
            stack.append((cur, num))
            cur, num = [], 0
        elif ch == "]":
            prev, k = stack.pop()
            prev.append("".join(cur) * k)
            cur = prev
        else:
            cur.append(ch)
    return "".join(cur)


if __name__ == "__main__":
    assert decode_string("3[a]2[bc]") == "aaabcbc"
    assert decode_string("3[a2[c]]") == "accaccacc"
    assert decode_string("2[abc]3[cd]ef") == "abcabccdcdcdef"
    assert decode_string("10[a]") == "a" * 10
    print("ok")
