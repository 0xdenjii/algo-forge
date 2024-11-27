# Graphs (Python)

Graph problems and classic algorithms, one per file. Each file runs standalone: `python <file>` executes its demo/asserts.

| File | Description |
| --- | --- |
| [0127_word_ladder.py](0127_word_ladder.py) | Shortest transformation sequence length between words (BFS with wildcard buckets). |
| [0130_surrounded_regions.py](0130_surrounded_regions.py) | Capture 'O' regions not connected to the border. |
| [0133_clone_graph.py](0133_clone_graph.py) | Deep-copy an undirected graph via DFS with a memo map. |
| [0200_number_of_islands.py](0200_number_of_islands.py) | Count connected land regions in a grid (BFS flood fill). |
| [0207_course_schedule.py](0207_course_schedule.py) | Detect whether a prerequisite graph has a cycle (Kahn's algorithm). |
| [0210_course_schedule_ii.py](0210_course_schedule_ii.py) | Return a valid course order (topological sort) or [] if cyclic. |
| [0261_graph_valid_tree.py](0261_graph_valid_tree.py) | Check whether edges form a single tree (n-1 edges, no cycle). |
| [0269_alien_dictionary.py](0269_alien_dictionary.py) | Derive letter order from a sorted alien word list (topological sort). |
| [0310_minimum_height_trees.py](0310_minimum_height_trees.py) | Roots that minimize tree height (peel leaves layer by layer). |
| [0329_longest_increasing_path_in_matrix.py](0329_longest_increasing_path_in_matrix.py) | Longest strictly increasing path in a matrix (DFS + memo on a DAG). |
| [0332_reconstruct_itinerary.py](0332_reconstruct_itinerary.py) | Lexicographically smallest Eulerian path of tickets (Hierholzer). |
| [0399_evaluate_division.py](0399_evaluate_division.py) | Answer ratio queries over a weighted graph of equations (DFS). |
| [0417_pacific_atlantic_water_flow.py](0417_pacific_atlantic_water_flow.py) | Cells that can drain to both oceans (reverse BFS from borders). |
| [0542_01_matrix.py](0542_01_matrix.py) | Distance of each cell to the nearest 0 (multi-source BFS). |
| [0547_number_of_provinces.py](0547_number_of_provinces.py) | Count connected components of an adjacency matrix (union-find). |
| [0684_redundant_connection.py](0684_redundant_connection.py) | Find the edge that creates a cycle in a tree-plus-one-edge (union-find). |
| [0695_max_area_of_island.py](0695_max_area_of_island.py) | Largest island area in a binary grid (iterative DFS). |
| [0721_accounts_merge.py](0721_accounts_merge.py) | Merge accounts that share an email (union-find over emails). |
| [0743_network_delay_time.py](0743_network_delay_time.py) | Time for a signal to reach all nodes (Dijkstra). |
| [0752_open_the_lock.py](0752_open_the_lock.py) | Fewest wheel turns to reach a lock code avoiding deadends (BFS). |
| [0778_swim_in_rising_water.py](0778_swim_in_rising_water.py) | Minimum time to swim across a grid (Dijkstra on max-height). |
| [0785_is_graph_bipartite.py](0785_is_graph_bipartite.py) | Two-color an undirected graph with BFS. |
| [0787_cheapest_flights_within_k_stops.py](0787_cheapest_flights_within_k_stops.py) | Cheapest price with at most k stops (bounded Bellman-Ford). |
| [0797_all_paths_from_source_to_target.py](0797_all_paths_from_source_to_target.py) | Enumerate all paths in a DAG from 0 to n-1 (backtracking). |
| [0802_find_eventual_safe_states.py](0802_find_eventual_safe_states.py) | Nodes whose every path leads to a terminal node (3-color DFS). |
| [0841_keys_and_rooms.py](0841_keys_and_rooms.py) | Check whether all rooms are reachable from room 0 (DFS). |
| [0994_rotting_oranges.py](0994_rotting_oranges.py) | Minutes until all oranges rot (multi-source BFS). |
| [1091_shortest_path_in_binary_matrix.py](1091_shortest_path_in_binary_matrix.py) | Shortest 8-directional clear path in a grid (BFS). |
| [1192_critical_connections.py](1192_critical_connections.py) | Find bridges in an undirected graph (Tarjan low-link). |
| [1514_path_with_maximum_probability.py](1514_path_with_maximum_probability.py) | Most probable path between two nodes (max-heap Dijkstra). |
| [1584_min_cost_to_connect_all_points.py](1584_min_cost_to_connect_all_points.py) | Minimum spanning tree over points with Manhattan distance (Prim). |
| [1631_path_with_minimum_effort.py](1631_path_with_minimum_effort.py) | Path minimizing max adjacent height difference (Dijkstra). |
| [a_star_grid_search.py](a_star_grid_search.py) | Shortest path on a grid with a Manhattan heuristic (A*). |
| [bellman_ford_shortest_path.py](bellman_ford_shortest_path.py) | Single-source shortest paths with negative edges (Bellman-Ford). |
| [edmonds_karp_max_flow.py](edmonds_karp_max_flow.py) | Maximum flow in a network using BFS augmenting paths (Edmonds-Karp). |
| [floyd_warshall_all_pairs.py](floyd_warshall_all_pairs.py) | All-pairs shortest paths with negative-cycle detection (Floyd-Warshall). |
| [hopcroft_karp_bipartite_matching.py](hopcroft_karp_bipartite_matching.py) | Maximum bipartite matching in O(E sqrt V) (Hopcroft-Karp). |
| [kosaraju_strongly_connected_components.py](kosaraju_strongly_connected_components.py) | SCCs via two DFS passes over the graph and its transpose (Kosaraju). |
| [kruskal_minimum_spanning_tree.py](kruskal_minimum_spanning_tree.py) | Minimum spanning tree via sorted edges and union-find (Kruskal). |
| [tarjan_strongly_connected_components.py](tarjan_strongly_connected_components.py) | Strongly connected components in a directed graph (Tarjan). |
