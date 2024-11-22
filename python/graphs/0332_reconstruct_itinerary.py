# LeetCode 332 - Reconstruct Itinerary
# Given airline tickets [from, to], reconstruct the itinerary starting at "JFK" that uses
# every ticket once; prefer the lexicographically smallest itinerary.
# Approach: Hierholzer's algorithm with min-heaps of destinations, appending on backtrack.
# Time: O(E log E)  Space: O(E)

import heapq
from collections import defaultdict


def find_itinerary(tickets: list[list[str]]) -> list[str]:
    adj = defaultdict(list)
    for a, b in tickets:
        heapq.heappush(adj[a], b)
    route, stack = [], ["JFK"]
    while stack:
        while adj[stack[-1]]:
            stack.append(heapq.heappop(adj[stack[-1]]))
        route.append(stack.pop())
    return route[::-1]


if __name__ == "__main__":
    t = [["JFK", "SFO"], ["JFK", "ATL"], ["SFO", "ATL"], ["ATL", "JFK"], ["ATL", "SFO"]]
    assert find_itinerary(t) == ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
    assert find_itinerary([["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]]) == ["JFK", "NRT", "JFK", "KUL"]
    print("itinerary:", find_itinerary(t))
