# Classic - 0/1 Knapsack (with item reconstruction)
# Problem: Given items with weights and values and a capacity W, choose a subset
#   (each item at most once) maximising total value without exceeding W.
#   Also return which items were chosen.
# Approach: Full table dp[i][w] = max(dp[i-1][w], dp[i-1][w-wi] + vi), then walk
#   back from dp[n][W] to recover the chosen items.
# Time: O(n * W)  Space: O(n * W)


def knapsack(weights: list[int], values: list[int], capacity: int) -> tuple[int, list[int]]:
    n = len(weights)
    dp = [[0] * (capacity + 1) for _ in range(n + 1)]
    for i in range(1, n + 1):
        wi, vi = weights[i - 1], values[i - 1]
        prev, row = dp[i - 1], dp[i]
        for w in range(capacity + 1):
            row[w] = prev[w]
            if wi <= w and prev[w - wi] + vi > row[w]:
                row[w] = prev[w - wi] + vi
    chosen, w = [], capacity
    for i in range(n, 0, -1):
        if dp[i][w] != dp[i - 1][w]:
            chosen.append(i - 1)
            w -= weights[i - 1]
    return dp[n][capacity], chosen[::-1]


if __name__ == "__main__":
    best, items = knapsack([1, 3, 4, 5], [1, 4, 5, 7], 7)
    assert best == 9 and sorted(items) == [1, 2]
    assert knapsack([5], [10], 4) == (0, [])
    print("ok", best, items)
