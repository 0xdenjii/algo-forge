# Segment Trees & Fenwick Trees (Rust)

Each file holds one problem and runs on its own: `rustc --edition 2021 -O file.rs && ./file`.
Each `main` asserts the problem's sample cases, and several also cross-check against a brute-force version.

## LeetCode

| File | Description |
|---|---|
| `0307_range_sum_query_mutable.rs` | Point update plus range sum, using a Fenwick tree built in O(n). |
| `0308_range_sum_query_2d_mutable.rs` | 2D Fenwick tree with rectangle sums by inclusion-exclusion. |
| `0315_count_of_smaller_numbers_after_self.rs` | Right-to-left sweep over compressed values with a Fenwick tree. |
| `0327_count_of_range_sum.rs` | Counts prefix-sum pairs inside a value window using a Fenwick tree. |
| `0406_queue_reconstruction_by_height.rs` | Finds the k-th empty slot by Fenwick binary lifting. |
| `0493_reverse_pairs.rs` | Counts pairs with nums[i] > 2*nums[j] using a Fenwick tree and binary search. |
| `0673_number_of_longest_increasing_subsequence.rs` | Fenwick tree over (length, count) pairs. |
| `0699_falling_squares.rs` | Range chmax plus range max with permanent (non-pushed) tags. |
| `0715_range_module.rs` | Dynamic segment tree with lazy assignment over [1, 1e9). |
| `0729_my_calendar_i.rs` | Dynamic range-add / range-max tree that rejects any overlap. |
| `0731_my_calendar_ii.rs` | Same tree, rejecting triple bookings. |
| `0732_my_calendar_iii.rs` | Dynamic range add; the root's max is the answer. |
| `0850_rectangle_area_ii.rs` | Sweep line with a coverage-count segment tree. |
| `1157_online_majority_element_in_subarray.rs` | Boyer-Moore voting segment tree plus position lists. |
| `1395_count_number_of_teams.rs` | Left and right Fenwick trees around the middle soldier. |
| `1505_minimum_possible_integer_after_at_most_k_adjacent_swaps.rs` | Greedy digit choice; a Fenwick tree tracks the shift cost. |
| `1649_create_sorted_array_through_instructions.rs` | Value-indexed Fenwick tree, cost min(less, greater). |
| `2179_count_good_triplets_in_an_array.rs` | Maps through positions, then counts increasing triplets with a Fenwick tree. |
| `2286_booking_concert_tickets_in_groups.rs` | Min/sum segment tree with descent and an amortized fill pointer. |
| `2407_longest_increasing_subsequence_ii.rs` | Iterative max segment tree indexed by value. |
| `2426_number_of_pairs_satisfying_inequality.rs` | Rewrites the condition as a[i] <= a[j] + diff, then uses a Fenwick tree. |
| `2940_find_building_where_alice_and_bob_can_meet.rs` | Max segment tree that finds the first index greater than a value. |

## Classic techniques and judge problems

| File | Description |
|---|---|
| `fenwick_range_update_range_query.rs` | Two Fenwick trees for range add plus range sum. |
| `fenwick_order_statistics_multiset.rs` | Multiset with insert, erase, rank and k-th via Fenwick binary lifting. |
| `segment_tree_lazy_range_add_range_sum.rs` | Recursive lazy propagation: range add, range sum. |
| `segment_tree_lazy_range_assign_range_min.rs` | Lazy assignment tags: range assign, range min. |
| `generic_segment_tree_monoid.rs` | Generic iterative tree over a `Monoid` trait (sum, min, gcd, concat). |
| `spoj_mkthnum_persistent_segment_tree_kth_smallest.rs` | Persistent segment tree for the k-th smallest in a range. |
| `merge_sort_tree_count_less_than.rs` | Merge sort tree counting values < x in a range. |
| `spoj_gss3_max_subarray_sum_with_updates.rs` | Max subarray sum with point updates (sum/prefix/suffix/best). |
| `inversion_count_fenwick.rs` | Inversion count with a Fenwick tree (SPOJ INVCNT). |
| `spoj_dquery_distinct_elements_in_range.rs` | Offline distinct-count queries using last occurrences in a Fenwick tree. |
| `li_chao_tree_min_lines.rs` | Li Chao tree: insert lines, query the min at x. |
| `cf_339d_xenia_and_bit_operations.rs` | Segment tree that alternates OR and XOR by level. |
| `cf_380c_sereja_and_brackets.rs` | Longest correct bracket subsequence in a range. |
| `cses_1143_hotel_queries.rs` | Max-tree descent to the first hotel with enough rooms. |
| `cses_1144_salary_queries.rs` | Offline compression plus a Fenwick tree for value-range counts with updates. |
