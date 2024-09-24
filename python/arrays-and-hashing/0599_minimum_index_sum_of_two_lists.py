# Problem: LeetCode 599 - Minimum Index Sum of Two Lists
#
# Find the common strings of two lists with the least index sum; return all ties.
#
# Approach: Index the first list in a dict, scan the second tracking the best sum.
# Complexity: O(n + m) time, O(n) space

def find_restaurant(list1: list[str], list2: list[str]) -> list[str]:
    index = {s: i for i, s in enumerate(list1)}
    best, out = float("inf"), []
    for j, s in enumerate(list2):
        if s in index:
            total = index[s] + j
            if total < best:
                best, out = total, [s]
            elif total == best:
                out.append(s)
    return out


if __name__ == "__main__":
    assert find_restaurant(["Shogun", "Tapioca Express", "Burger King", "KFC"],
                           ["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"]) == ["Shogun"]
    assert find_restaurant(["happy", "sad", "good"], ["sad", "happy", "good"]) == ["sad", "happy"]
    print("ok")
