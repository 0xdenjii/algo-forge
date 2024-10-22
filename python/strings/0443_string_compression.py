# LeetCode 443 - String Compression
# Problem: Compress a char list in place: each run becomes the char
#   followed by its count (if > 1). Return the new length.
# Approach: Read pointer scans runs; write pointer emits char + digits.
# Time: O(n)  Space: O(1)


def compress(chars: list[str]) -> int:
    write = read = 0
    while read < len(chars):
        ch, start = chars[read], read
        while read < len(chars) and chars[read] == ch:
            read += 1
        chars[write] = ch
        write += 1
        if read - start > 1:
            for d in str(read - start):
                chars[write] = d
                write += 1
    return write


if __name__ == "__main__":
    c = list("aabbccc")
    assert compress(c) == 6 and c[:6] == list("a2b2c3")
    c = list("a")
    assert compress(c) == 1
    c = list("abbbbbbbbbbbb")
    assert compress(c) == 4 and c[:4] == list("ab12")
    print("ok")
