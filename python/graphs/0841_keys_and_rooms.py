# LeetCode 841 - Keys and Rooms
# Room i contains keys rooms[i] to other rooms. Starting in unlocked room 0,
# return True if every room can be visited.
# Approach: DFS/graph reachability from node 0.
# Time: O(V + E)  Space: O(V)

def can_visit_all_rooms(rooms: list[list[int]]) -> bool:
    seen = {0}
    stack = [0]
    while stack:
        for key in rooms[stack.pop()]:
            if key not in seen:
                seen.add(key)
                stack.append(key)
    return len(seen) == len(rooms)


if __name__ == "__main__":
    assert can_visit_all_rooms([[1], [2], [3], []])
    assert not can_visit_all_rooms([[1, 3], [3, 0, 1], [2], [0]])
    print("all rooms:", can_visit_all_rooms([[1], [2], [3], []]))
