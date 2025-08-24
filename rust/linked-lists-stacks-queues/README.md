# Linked Lists, Stacks & Queues (Rust)

Each file is a standalone program: `rustc --edition 2021 <file>.rs && ./<file>` (prints `ok`).

## Linked lists
| File | Description |
|---|---|
| 0002_add_two_numbers.rs | Add two reversed-digit lists with carry |
| 0019_remove_nth_node_from_end.rs | Unlink the n-th node from the end with a mutable cursor |
| 0021_merge_two_sorted_lists.rs | Splice two sorted lists using a tail slot cursor |
| 0023_merge_k_sorted_lists.rs | Min-heap merge of k sorted lists |
| 0024_swap_nodes_in_pairs.rs | Recursive pairwise node swap |
| 0025_reverse_nodes_in_k_group.rs | Reverse each full block of k nodes |
| 0061_rotate_list.rs | Rotate right by k via cut and reattach |
| 0082_remove_duplicates_from_sorted_list_ii.rs | Drop every value that appears more than once |
| 0083_remove_duplicates_from_sorted_list.rs | Keep one copy of each value in a sorted list |
| 0086_partition_list.rs | Stable partition around x into two chains |
| 0092_reverse_linked_list_ii.rs | Reverse the sublist between positions left and right |
| 0142_linked_list_cycle_ii.rs | Floyd cycle detection and entry point on an index arena |
| 0143_reorder_list.rs | Split, reverse second half, interleave |
| 0148_sort_list.rs | Merge sort on a linked list |
| 0203_remove_linked_list_elements.rs | Remove all nodes with a given value |
| 0206_reverse_linked_list.rs | Iterative in-place reversal |
| 0234_palindrome_linked_list.rs | Palindrome check by reversing the second half |
| 0328_odd_even_linked_list.rs | Group odd-position nodes before even-position nodes |
| 0445_add_two_numbers_ii.rs | Add most-significant-first lists using stacks |
| 0876_middle_of_the_linked_list.rs | Slow/fast pointers over shared references |
| generic_linked_stack.rs | Generic linked stack with iterators and iterative Drop |

## Stacks
| File | Description |
|---|---|
| 0020_valid_parentheses.rs | Bracket matching with an expected-closer stack |
| 0042_trapping_rain_water.rs | Trapped water by filling layers on a monotonic stack |
| 0071_simplify_path.rs | Canonical Unix path with a directory stack |
| 0084_largest_rectangle_in_histogram.rs | Largest rectangle via increasing stack with sentinel |
| 0150_evaluate_reverse_polish_notation.rs | RPN evaluator with Result-based error handling |
| 0155_min_stack.rs | O(1) min stack storing (value, running min) |
| 0224_basic_calculator.rs | +, -, parentheses and unary minus with a sign stack |
| 0394_decode_string.rs | Nested k[...] decoding with a (prefix, count) stack |
| 0496_next_greater_element_i.rs | Next greater lookup map built with a monotonic stack |
| 0503_next_greater_element_ii.rs | Next greater in a circular array (two passes) |
| 0735_asteroid_collision.rs | Collision simulation with a survivor stack |
| 0739_daily_temperatures.rs | Days until warmer with a decreasing stack |
| 0853_car_fleet.rs | Fleet counting with a stack of arrival times |
| 0901_online_stock_span.rs | Streaming span with merged (price, span) stack |
| 0907_sum_of_subarray_minimums.rs | Contribution technique with previous/next smaller stacks |
| 1047_remove_all_adjacent_duplicates_in_string.rs | String used as a stack to cancel adjacent pairs |

## Queues & design
| File | Description |
|---|---|
| 0146_lru_cache.rs | LRU cache: HashMap plus Vec-arena doubly linked list |
| 0225_implement_stack_using_queues.rs | Stack on one VecDeque by rotating after push |
| 0232_implement_queue_using_stacks.rs | Amortized O(1) queue with inbox/outbox stacks |
| 0239_sliding_window_maximum.rs | Monotonic deque window maximum |
| 0622_design_circular_queue.rs | Fixed-capacity generic ring buffer |
