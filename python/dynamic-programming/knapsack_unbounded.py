# Classic - Unbounded Knapsack
# Problem: Same as 0/1 knapsack but every item may be taken any number of times.
#   Maximise total value within capacity W.
# Approach: dp[w] = max(dp[w], dp[w - wi] + vi) iterating w upwards so an item can
#   be reused within the same pass.
# Time: O(n * W)  Space: O(W)


def unbounded_knapsack(weights: list[int], values: list[int], capacity: int) -> int:
    dp = [0] * (capacity + 1)
    for wi, vi in zip(weights, values):
        for w in range(wi, capacity + 1):
            dp[w] = max(dp[w], dp[w - wi] + vi)
    return dp[capacity]


if __name__ == "__main__":
    assert unbounded_knapsack([1, 3, 4, 5], [10, 40, 50, 70], 8) == 110
    assert unbounded_knapsack([5, 10, 15], [10, 30, 20], 100) == 300
    print("ok")
