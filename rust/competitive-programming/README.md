# Competitive Programming in Rust

One problem per file. Each file is a standalone program: `rustc --edition 2021 -O file.rs && ./file`.
Every `main` asserts against the known sample answers.

## Templates and number theory
| File | Description |
|---|---|
| `fast_io_scanner.rs` | Token scanner over the whole of stdin, with a buffered writer, for contest I/O |
| `modular_arithmetic.rs` | Fast power, Fermat inverse, factorial-based nCr mod 1e9+7 |
| `sieve_of_eratosthenes.rs` | Linear sieve with smallest-prime-factor table and O(log x) factorization |
| `extended_gcd_crt.rs` | Extended Euclid and CRT for non-coprime moduli |
| `matrix_exponentiation_fibonacci.rs` | Matrix power for F(n) mod p with n up to 1e18 |

## Arrays, prefix sums, and stacks/queues
| File | Description |
|---|---|
| `0042_trapping_rain_water.rs` | LC 42: two-pointer trapped water |
| `0084_largest_rectangle_in_histogram.rs` | LC 84: monotonic stack |
| `0239_sliding_window_maximum.rs` | LC 239: monotonic deque |
| `0304_range_sum_query_2d_prefix_sums.rs` | LC 304: 2D prefix sums |
| `1109_corporate_flight_bookings_difference_array.rs` | LC 1109: difference array for range adds |
| `0875_koko_eating_bananas_binary_search_answer.rs` | LC 875: binary search on the answer |
| `mo_algorithm_distinct.rs` | SPOJ DQUERY: Mo's algorithm for distinct values in a range |

## Range-query data structures
| File | Description |
|---|---|
| `0307_range_sum_query_mutable_fenwick.rs` | LC 307: Fenwick tree, plus inversion counting |
| `segment_tree_lazy.rs` | Segment tree with lazy range add and range sum |
| `sparse_table_rmq.rs` | Sparse table with O(1) range-minimum queries |
| `0547_number_of_provinces_dsu.rs` | LC 547: union-find with path halving and union by size |

## Graphs
| File | Description |
|---|---|
| `0743_network_delay_time_dijkstra.rs` | LC 743: Dijkstra with a binary heap |
| `bellman_ford_negative_cycle.rs` | Bellman-Ford with negative-cycle detection |
| `1334_find_the_city_floyd_warshall.rs` | LC 1334: Floyd-Warshall all-pairs shortest paths |
| `0210_course_schedule_ii_toposort.rs` | LC 210: Kahn's topological sort |
| `1584_min_cost_connect_points_kruskal.rs` | LC 1584: Kruskal MST |
| `tarjan_scc.rs` | Iterative Tarjan strongly connected components |
| `1192_critical_connections_bridges.rs` | LC 1192: bridges via low-link (handles multi-edges) |
| `lca_binary_lifting.rs` | LCA and tree distance via binary lifting |
| `0847_shortest_path_visiting_all_nodes_bitmask.rs` | LC 847: BFS over (node, mask) states |
| `dinic_max_flow.rs` | Dinic maximum flow |
| `two_sat.rs` | 2-SAT using Kosaraju SCC on the implication graph |

## Dynamic programming
| File | Description |
|---|---|
| `0300_longest_increasing_subsequence.rs` | LC 300: O(n log n) patience-sorting LIS |
| `0072_edit_distance.rs` | LC 72: Levenshtein DP with one rolling row |
| `knapsack_01.rs` | AtCoder DP D: 0/1 knapsack |
| `0322_coin_change.rs` | LC 322: unbounded coin DP |
| `0312_burst_balloons_interval_dp.rs` | LC 312: interval DP over which balloon bursts last |
| `0233_number_of_digit_one_digit_dp.rs` | LC 233: digit DP |
| `0051_n_queens_bitmask.rs` | LC 51: bitmask backtracking |

## Strings
| File | Description |
|---|---|
| `0028_find_first_occurrence_kmp.rs` | LC 28: KMP prefix function, finds all matches |
| `2223_sum_of_scores_z_function.rs` | LC 2223: Z-function |
| `1044_longest_duplicate_substring_rolling_hash.rs` | LC 1044: binary search with a double rolling hash, checked byte by byte |
| `0005_longest_palindromic_substring_manacher.rs` | LC 5: Manacher's algorithm |
| `0212_word_search_ii_trie.rs` | LC 212: trie-guided grid DFS |

## Geometry
| File | Description |
|---|---|
| `0587_erect_the_fence_convex_hull.rs` | LC 587: monotone-chain convex hull that keeps collinear points |
