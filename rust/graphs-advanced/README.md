# Rust - Advanced Graphs

Each file is a standalone program: `rustc --edition 2021 <file>.rs && ./<file>` runs its assertions.

| File | Description |
|---|---|
| `0127_word_ladder_bidirectional_bfs.rs` | Word Ladder: bidirectional BFS over one-letter transformations |
| `0210_course_schedule_ii_kahn.rs` | Course Schedule II: topological order via Kahn's algorithm |
| `0269_alien_dictionary.rs` | Alien Dictionary: derive letter order from sorted words, topo sort |
| `0310_minimum_height_trees.rs` | Minimum Height Trees: find tree centers by leaf peeling |
| `0332_reconstruct_itinerary_hierholzer.rs` | Reconstruct Itinerary: lexicographically smallest Eulerian path (Hierholzer) |
| `0743_network_delay_time.rs` | Network Delay Time: Dijkstra with a binary heap |
| `0778_swim_in_rising_water.rs` | Swim in Rising Water: minimax-path Dijkstra on a grid |
| `0785_is_graph_bipartite.rs` | Is Graph Bipartite: BFS 2-coloring |
| `0787_cheapest_flights_within_k_stops.rs` | Cheapest Flights Within K Stops: round-limited Bellman-Ford |
| `0802_find_eventual_safe_states.rs` | Find Eventual Safe States: reverse-graph Kahn from terminals |
| `0815_bus_routes.rs` | Bus Routes: BFS over buses/stops |
| `0834_sum_of_distances_in_tree.rs` | Sum of Distances in Tree: rerooting DP |
| `0847_shortest_path_visiting_all_nodes.rs` | Shortest Path Visiting All Nodes: BFS over (node, bitmask) |
| `1192_critical_connections_bridges.rs` | Critical Connections: iterative Tarjan bridge finding |
| `1334_find_the_city_floyd_warshall.rs` | Find the City: Floyd-Warshall all-pairs shortest paths |
| `1368_min_cost_valid_path_01bfs.rs` | Min Cost to Make a Valid Grid Path: 0-1 BFS with a deque |
| `1483_kth_ancestor_of_tree_node.rs` | Kth Ancestor of a Tree Node: binary lifting |
| `1584_min_cost_connect_points_prim.rs` | Min Cost to Connect All Points: dense O(n^2) Prim |
| `1631_path_with_minimum_effort.rs` | Path With Minimum Effort: Kruskal-style union-find on sorted edges |
| `a_star_grid_pathfinding.rs` | A* search on a grid with Manhattan heuristic and path reconstruction |
| `articulation_points.rs` | Cut vertices via DFS low-link |
| `dinic_max_flow.rs` | Max flow with Dinic's level graph + blocking flow |
| `edmonds_karp_max_flow.rs` | Max flow with Edmonds-Karp, plus min-cut source side |
| `hopcroft_karp_bipartite_matching.rs` | Maximum bipartite matching in O(E sqrt V) |
| `kosaraju_scc.rs` | Strongly connected components via Kosaraju (two-pass DFS) |
| `kruskal_mst_union_find.rs` | Minimum spanning tree via Kruskal + DSU |
| `lca_binary_lifting.rs` | Lowest common ancestor and tree distance via binary lifting |
| `min_cost_max_flow_spfa.rs` | Min-cost max-flow with successive shortest paths (SPFA) |
| `tarjan_scc.rs` | Strongly connected components via iterative Tarjan |
| `two_sat.rs` | 2-SAT solver using implication graph SCCs |
