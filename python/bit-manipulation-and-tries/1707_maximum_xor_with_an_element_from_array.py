# Problem: LeetCode 1707 - Maximum XOR With an Element From Array
#
# For each query (x, m), return max(x ^ num) over nums with num <= m, or -1.
#
# Approach: Offline: sort nums and queries by m, insert nums <= m into a binary trie, then
#   greedy-query.
# Complexity: O((n + q) log n + 30(n + q)) time, O(30 n) space

def maximize_xor(nums: list[int], queries: list[list[int]]) -> list[int]:
    nums = sorted(nums)
    order = sorted(range(len(queries)), key=lambda i: queries[i][1])
    children = [[0, 0]]
    ans = [-1] * len(queries)
    j = 0
    for qi in order:
        x, m = queries[qi]
        while j < len(nums) and nums[j] <= m:
            node = 0
            for b in range(29, -1, -1):
                bit = nums[j] >> b & 1
                if not children[node][bit]:
                    children.append([0, 0])
                    children[node][bit] = len(children) - 1
                node = children[node][bit]
            j += 1
        if j == 0:
            continue
        node, res = 0, 0
        for b in range(29, -1, -1):
            want = (x >> b & 1) ^ 1
            if children[node][want]:
                res |= 1 << b
                node = children[node][want]
            else:
                node = children[node][want ^ 1]
        ans[qi] = res
    return ans


if __name__ == "__main__":
    assert maximize_xor([0, 1, 2, 3, 4], [[3, 1], [1, 3], [5, 6]]) == [3, 3, 7]
    assert maximize_xor([5, 2, 4, 6, 6, 3], [[12, 4], [8, 1], [6, 3]]) == [15, -1, 5]
    print("ok")
