# LeetCode 271 - Encode and Decode Strings
# Problem: Design encode(list[str]) -> str and decode(str) -> list[str]
#   that round-trip any strings, including ones containing delimiters.
# Approach: Length-prefix each string as "<len>#<payload>".
# Time: O(total length)  Space: O(total length)


class Codec:
    def encode(self, strs: list[str]) -> str:
        return "".join(f"{len(s)}#{s}" for s in strs)

    def decode(self, data: str) -> list[str]:
        out, i = [], 0
        while i < len(data):
            j = data.index("#", i)
            n = int(data[i:j])
            out.append(data[j + 1:j + 1 + n])
            i = j + 1 + n
        return out


if __name__ == "__main__":
    c = Codec()
    for case in (["hello", "world"], [""], [], ["a#b", "12#", "#"]):
        assert c.decode(c.encode(case)) == case
    print("ok")
