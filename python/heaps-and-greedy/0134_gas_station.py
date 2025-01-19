"""LeetCode 134 - Gas Station

Stations on a circle give gas[i] and cost cost[i] to reach the next one.
Return the start index allowing a full loop, or -1 (answer is unique).

Approach: if total gas >= total cost a solution exists; whenever the running
tank goes negative, no start up to here works, so restart after it.
Time: O(n)  Space: O(1)
"""


def can_complete_circuit(gas: list[int], cost: list[int]) -> int:
    total = tank = start = 0
    for i, (g, c) in enumerate(zip(gas, cost)):
        total += g - c
        tank += g - c
        if tank < 0:
            start, tank = i + 1, 0
    return start if total >= 0 else -1


if __name__ == "__main__":
    assert can_complete_circuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]) == 3
    assert can_complete_circuit([2, 3, 4], [3, 4, 3]) == -1
    print("ok")
