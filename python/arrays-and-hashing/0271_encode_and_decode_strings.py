# Problem: LeetCode 271 - Encode and Decode Strings
#
# Design encode/decode so a list of arbitrary strings round-trips through a
# single string.
#
# Approach: Length-prefix each string: '<len>#<payload>'. Decoding reads digits up to '#'.
# Complexity: O(total length) time and space

def encode(strs: list[str]) -> str:
    return "".join(f"{len(s)}#{s}" for s in strs)


def decode(data: str) -> list[str]:
    out: list[str] = []
    i = 0
    while i < len(data):
        j = data.index("#", i)
        n = int(data[i:j])
        out.append(data[j + 1:j + 1 + n])
        i = j + 1 + n
    return out


if __name__ == "__main__":
    for case in (["lint", "code", "love", "you"], ["", "#", "12#ab"], []):
        assert decode(encode(case)) == case
    print("ok")
