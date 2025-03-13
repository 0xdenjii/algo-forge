"""Fractional Knapsack (classic greedy)

Items have value and weight; fractions of items may be taken. Maximise the
total value within a weight capacity.

Approach: take items in decreasing value/weight ratio, splitting the last.
Time: O(n log n)  Space: O(n)
"""


def fractional_knapsack(items: list[tuple[float, float]], capacity: float) -> float:
    """items: list of (value, weight) with weight > 0."""
    total = 0.0
    for value, weight in sorted(items, key=lambda it: it[0] / it[1], reverse=True):
        if capacity <= 0:
            break
        take = min(weight, capacity)
        total += value * take / weight
        capacity -= take
    return total


if __name__ == "__main__":
    assert abs(fractional_knapsack([(60, 10), (100, 20), (120, 30)], 50) - 240.0) < 1e-9
    assert fractional_knapsack([(10, 5)], 0) == 0.0
    print("ok")
