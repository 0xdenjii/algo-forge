# LeetCode 399 - Evaluate Division
# Given equations a / b = k, answer queries c / d, returning -1.0 when undeterminable.
# Approach: build a weighted graph a->b (k) and b->a (1/k); DFS multiplies edge weights.
# Time: O(Q * (V + E))  Space: O(V + E)

from collections import defaultdict


def calc_equation(equations, values, queries) -> list[float]:
    g = defaultdict(dict)
    for (a, b), k in zip(equations, values):
        g[a][b] = k
        g[b][a] = 1.0 / k

    def query(src, dst):
        if src not in g or dst not in g:
            return -1.0
        stack, seen = [(src, 1.0)], {src}
        while stack:
            u, acc = stack.pop()
            if u == dst:
                return acc
            for v, w in g[u].items():
                if v not in seen:
                    seen.add(v)
                    stack.append((v, acc * w))
        return -1.0

    return [query(a, b) for a, b in queries]


if __name__ == "__main__":
    res = calc_equation([["a", "b"], ["b", "c"]], [2.0, 3.0],
                        [["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]])
    assert [round(x, 5) for x in res] == [6.0, 0.5, -1.0, 1.0, -1.0]
    print("answers:", res)
