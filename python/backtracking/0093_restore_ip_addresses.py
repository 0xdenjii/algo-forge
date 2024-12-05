"""
LeetCode 93 - Restore IP Addresses
Problem: Given a digit string, return all valid IPv4 addresses formed by
inserting three dots. Each octet is 0-255 with no leading zeros.
Approach: DFS choosing 1-3 digits per octet; prune when the remaining length
cannot fit the remaining octets.
Time: O(3^4)  Space: O(1)
"""
from typing import List


def restore_ip_addresses(s: str) -> List[str]:
    out: List[str] = []
    parts: List[str] = []

    def dfs(i: int) -> None:
        left = 4 - len(parts)
        rest = len(s) - i
        if left == 0:
            if rest == 0:
                out.append(".".join(parts))
            return
        if not (left <= rest <= 3 * left):
            return
        for size in range(1, 4):
            seg = s[i:i + size]
            if len(seg) < size or (seg[0] == "0" and size > 1) or int(seg) > 255:
                break
            parts.append(seg)
            dfs(i + size)
            parts.pop()

    dfs(0)
    return out


if __name__ == "__main__":
    assert restore_ip_addresses("25525511135") == ["255.255.11.135", "255.255.111.35"]
    assert restore_ip_addresses("0000") == ["0.0.0.0"]
    assert restore_ip_addresses("101023") == ["1.0.10.23", "1.0.102.3", "10.1.0.23",
                                              "10.10.2.3", "101.0.2.3"]
    print("ok")
