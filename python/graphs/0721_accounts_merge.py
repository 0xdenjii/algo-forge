# LeetCode 721 - Accounts Merge
# Each account is [name, email1, email2, ...]. Accounts sharing any email belong to the
# same person. Return merged accounts with emails sorted.
# Approach: union-find on emails; group by root and attach the owner's name.
# Time: O(N log N) for N total emails  Space: O(N)

from collections import defaultdict


def accounts_merge(accounts: list[list[str]]) -> list[list[str]]:
    parent: dict[str, str] = {}
    owner: dict[str, str] = {}

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for name, *emails in accounts:
        for e in emails:
            parent.setdefault(e, e)
            owner[e] = name
        for e in emails[1:]:
            parent[find(e)] = find(emails[0])
    groups = defaultdict(list)
    for e in parent:
        groups[find(e)].append(e)
    return sorted([owner[root]] + sorted(es) for root, es in groups.items())


if __name__ == "__main__":
    acc = [["John", "a@m.com", "b@m.com"], ["John", "c@m.com", "a@m.com"],
           ["Mary", "mary@m.com"], ["John", "z@m.com"]]
    assert accounts_merge(acc) == [["John", "a@m.com", "b@m.com", "c@m.com"],
                                   ["John", "z@m.com"], ["Mary", "mary@m.com"]]
    print(accounts_merge(acc))
