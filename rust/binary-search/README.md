# Binary Search (Rust)

Each file is a standalone program: `rustc --edition 2021 <file>.rs && ./<file>`. Every `main` runs assertions, and all of them pass.

| File | Description |
|---|---|
| 0004_median_of_two_sorted_arrays.rs | Median of two sorted arrays: binary search on the partition of the shorter array |
| 0033_search_in_rotated_sorted_array.rs | Find target in a rotated sorted array by locating the sorted half |
| 0034_find_first_and_last_position.rs | First and last index of target, using two partition points |
| 0035_search_insert_position.rs | Insert position (lower_bound) |
| 0069_sqrt_x.rs | Integer square root with overflow-safe bisection |
| 0074_search_a_2d_matrix.rs | Search a row-major sorted matrix as one flat array |
| 0081_search_in_rotated_sorted_array_ii.rs | Rotated search with duplicates (shrinks both ends when ambiguous) |
| 0153_find_minimum_in_rotated_sorted_array.rs | Minimum of a rotated array, comparing mid with the right end |
| 0154_find_minimum_in_rotated_sorted_array_ii.rs | Rotated minimum with duplicates |
| 0162_find_peak_element.rs | Any peak, found by following the rising slope |
| 0240_search_a_2d_matrix_ii.rs | Row/column-sorted matrix: per-row binary search with pruning |
| 0275_h_index_ii.rs | H-index on sorted citations in O(log n) |
| 0278_first_bad_version.rs | First true value of a monotone predicate API |
| 0287_find_the_duplicate_number.rs | Duplicate value via binary search on value range (pigeonhole) |
| 0300_longest_increasing_subsequence.rs | LIS in O(n log n) with patience sorting |
| 0354_russian_doll_envelopes.rs | 2D nesting: sort trick, then LIS |
| 0367_valid_perfect_square.rs | Perfect square check without sqrt |
| 0378_kth_smallest_element_in_a_sorted_matrix.rs | Kth smallest via value search and staircase counting |
| 0410_split_array_largest_sum.rs | Minimise the largest piece sum (binary search on the answer) |
| 0441_arranging_coins.rs | Complete staircase rows, using the largest k with k(k+1)/2 <= n |
| 0540_single_element_in_a_sorted_array.rs | Single element, found from pair-parity alignment |
| 0658_find_k_closest_elements.rs | K closest elements by searching the window's left edge |
| 0668_kth_smallest_number_in_multiplication_table.rs | Kth number in the m x n multiplication table |
| 0704_binary_search.rs | Classic exact-match binary search |
| 0719_find_kth_smallest_pair_distance.rs | Kth smallest pair distance (value search plus two pointers) |
| 0744_find_smallest_letter_greater_than_target.rs | upper_bound with wrap-around |
| 0875_koko_eating_bananas.rs | Minimum eating speed (binary search on the answer) |
| 0981_time_based_key_value_store.rs | Timestamped key-value store design with upper-bound lookups |
| 1011_capacity_to_ship_packages_within_d_days.rs | Minimum ship capacity, with a greedy feasibility check |
| 1095_find_in_mountain_array.rs | Find target in a mountain array: peak search, then two searches |
| 1283_smallest_divisor_given_a_threshold.rs | Smallest divisor that keeps the rounded-up sum within the threshold |
| 1482_minimum_number_of_days_to_make_m_bouquets.rs | Earliest day to make m bouquets of k adjacent flowers |
| 1539_kth_missing_positive_number.rs | Kth missing positive integer, using the missing-count function |
| 2187_minimum_time_to_complete_trips.rs | Minimum time for the buses to finish the required trips |
| 2300_successful_pairs_of_spells_and_potions.rs | Successful spell/potion pairs per spell, via lower_bound on sorted potions |
| aggressive_cows.rs | SPOJ AGGRCOW: maximise the minimum distance between cows |
| exponential_search.rs | Galloping search over a sequence of unknown size |
| generic_lower_upper_bound.rs | Generic lower_bound/upper_bound, checked against `partition_point` |
| real_valued_cube_root.rs | Bisection on reals with a fixed iteration count (cube root) |
