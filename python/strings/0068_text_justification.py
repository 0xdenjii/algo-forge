# LeetCode 68 - Text Justification
# Problem: Format words into lines of exactly maxWidth characters, fully
#   justified; the last line and single-word lines are left-justified.
# Approach: Greedily pack words per line, then spread extra spaces with
#   the leftmost gaps receiving one more.
# Time: O(total chars)  Space: O(total chars)


def full_justify(words: list[str], max_width: int) -> list[str]:
    lines, i = [], 0
    while i < len(words):
        j, length = i, 0
        while j < len(words) and length + len(words[j]) + (j - i) <= max_width:
            length += len(words[j])
            j += 1
        gaps = j - i - 1
        if j == len(words) or gaps == 0:
            line = " ".join(words[i:j]).ljust(max_width)
        else:
            q, r = divmod(max_width - length, gaps)
            parts = [w + " " * (q + (1 if k < r else 0)) for k, w in enumerate(words[i:j - 1])]
            line = "".join(parts) + words[j - 1]
        lines.append(line)
        i = j
    return lines


if __name__ == "__main__":
    assert full_justify(["This", "is", "an", "example", "of", "text", "justification."], 16) == [
        "This    is    an", "example  of text", "justification.  "]
    assert full_justify(["What", "must", "be", "acknowledgment", "shall", "be"], 16) == [
        "What   must   be", "acknowledgment  ", "shall be        "]
    print("ok")
