# Rust - Advanced Dynamic Programming

Each file is a standalone program: `rustc --edition 2021 <file>.rs && ./<file>`. Every `main` runs assertions.

| File | Description |
|---|---|
| 0010_regular_expression_matching.rs | Regex matching with `.` and `*` via 2D suffix DP |
| 0044_wildcard_matching.rs | Wildcard `?`/`*` matching with a rolling 1D DP |
| 0072_edit_distance.rs | Levenshtein distance in O(m) space |
| 0087_scramble_string.rs | 3D interval DP over length and start offsets |
| 0097_interleaving_string.rs | Interleaving check with a 1D DP row |
| 0115_distinct_subsequences.rs | Count subsequences of s equal to t |
| 0123_best_time_stock_iii.rs | Two-transaction stock profit, 4-state machine |
| 0132_palindrome_partitioning_ii.rs | Min palindrome cuts via center expansion |
| 0140_word_break_ii.rs | All dictionary sentences via memoized DFS |
| 0174_dungeon_game.rs | Minimum starting health with reverse grid DP |
| 0188_best_time_stock_iv.rs | k-transaction stock profit |
| 0300_longest_increasing_subsequence.rs | LIS in O(n log n) with patience sorting |
| 0312_burst_balloons.rs | Interval DP choosing the last balloon burst |
| 0337_house_robber_iii.rs | Tree DP with robbed/not-robbed pairs |
| 0354_russian_doll_envelopes.rs | 2D nesting reduced to LIS |
| 0410_split_array_largest_sum.rs | Partition DP minimizing the largest part |
| 0464_can_i_win.rs | Bitmask game DP (minimax with memo) |
| 0516_longest_palindromic_subsequence.rs | Interval DP rolled to O(n) space |
| 0546_remove_boxes.rs | 3D interval DP with attached-count state |
| 0629_k_inverse_pairs_array.rs | Permutations with k inversions, sliding-window sums |
| 0664_strange_printer.rs | Interval DP merging same-character turns |
| 0691_stickers_to_spell_word.rs | Bitmask DP over covered target letters |
| 0887_super_egg_drop.rs | Inverted DP: floors coverable with m moves |
| 0902_numbers_at_most_n_given_digit_set.rs | Digit counting with a tight-prefix walk |
| 0943_shortest_superstring.rs | Held-Karp overlap maximization with reconstruction |
| 0968_binary_tree_cameras.rs | Three-state greedy tree DP |
| 1000_minimum_cost_to_merge_stones.rs | Interval DP merging k piles at a time |
| 1125_smallest_sufficient_team.rs | Bitmask DP over skills storing team bitsets |
| 1235_maximum_profit_in_job_scheduling.rs | Weighted interval scheduling with binary search |
| bounded_knapsack_binary_split.rs | Bounded knapsack via power-of-two splitting |
| digit_dp_count_digit_sum.rs | Digit DP: count numbers <= N with a given digit sum |
| matrix_chain_multiplication.rs | Matrix chain order with parenthesization |
| sum_over_subsets.rs | SOS DP (subset zeta transform) |
| travelling_salesman_held_karp.rs | Held-Karp bitmask DP for TSP |
