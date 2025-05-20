# Problem: LeetCode 393 - UTF-8 Validation
#
# Given a list of byte values, decide whether it is a valid UTF-8 encoding
# (1-4 byte sequences, continuation bytes 10xxxxxx).
#
# Approach: Track remaining continuation bytes; classify leading bytes by high-bit patterns.
# Complexity: O(n) time, O(1) space

def valid_utf8(data: list[int]) -> bool:
    remaining = 0
    for byte in data:
        byte &= 0xFF
        if remaining:
            if byte >> 6 != 0b10:
                return False
            remaining -= 1
        elif byte >> 7 == 0:
            continue
        elif byte >> 5 == 0b110:
            remaining = 1
        elif byte >> 4 == 0b1110:
            remaining = 2
        elif byte >> 3 == 0b11110:
            remaining = 3
        else:
            return False
    return remaining == 0


if __name__ == "__main__":
    assert valid_utf8([197, 130, 1])
    assert not valid_utf8([235, 140, 4])
    assert valid_utf8(list("h\u00e9llo \u20ac".encode()))
    print("ok")
