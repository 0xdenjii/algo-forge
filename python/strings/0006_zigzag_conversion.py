# LeetCode 6 - Zigzag Conversion
# Problem: Write s in a zigzag over numRows rows, then read row by row.
# Approach: Walk rows bouncing between top and bottom, appending chars.
# Time: O(n)  Space: O(n)


def convert(s: str, num_rows: int) -> str:
    if num_rows == 1 or num_rows >= len(s):
        return s
    rows = [[] for _ in range(num_rows)]
    row, step = 0, 1
    for ch in s:
        rows[row].append(ch)
        if row == 0:
            step = 1
        elif row == num_rows - 1:
            step = -1
        row += step
    return "".join("".join(r) for r in rows)


if __name__ == "__main__":
    assert convert("PAYPALISHIRING", 3) == "PAHNAPLSIIGYIR"
    assert convert("PAYPALISHIRING", 4) == "PINALSIGYAHRPI"
    assert convert("A", 1) == "A"
    print("ok")
