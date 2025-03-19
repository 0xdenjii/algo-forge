# Classic (CLRS 15.1) - Rod Cutting
# Problem: A rod of length n can be cut into pieces; prices[i] is the price of a
#   piece of length i+1. Maximise revenue and report one optimal set of cuts.
# Approach: best[L] = max over first piece k of prices[k-1] + best[L-k]; store the
#   chosen first piece to rebuild the cuts.
# Time: O(n^2)  Space: O(n)


def cut_rod(prices: list[int], n: int) -> tuple[int, list[int]]:
    best = [0] * (n + 1)
    first = [0] * (n + 1)
    for length in range(1, n + 1):
        for k in range(1, min(length, len(prices)) + 1):
            cand = prices[k - 1] + best[length - k]
            if cand > best[length]:
                best[length], first[length] = cand, k
    cuts, length = [], n
    while length > 0 and first[length]:
        cuts.append(first[length])
        length -= first[length]
    return best[n], cuts


if __name__ == "__main__":
    prices = [1, 5, 8, 9, 10, 17, 17, 20]
    rev, cuts = cut_rod(prices, 8)
    assert rev == 22 and sum(cuts) == 8
    assert cut_rod(prices, 4)[0] == 10
    print("ok", rev, cuts)
