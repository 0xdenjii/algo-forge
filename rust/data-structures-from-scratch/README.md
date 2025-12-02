# Data Structures from Scratch (Rust)

Each file is a standalone program: `rustc --edition 2021 -O <file>.rs && ./<file>`. Every `main` asserts its own behavior.

| File | Description |
|---|---|
| `0146_lru_cache.rs` | LRU cache: HashMap plus an index-based doubly linked list, O(1) get/put |
| `0155_min_stack.rs` | Stack with O(1) get_min, using (value, running min) pairs |
| `0173_binary_search_tree_iterator.rs` | In-order BST iterator with an O(h) explicit stack |
| `0208_implement_trie.rs` | Prefix tree with insert, search and starts_with |
| `0211_design_add_and_search_words.rs` | Arena-backed trie with `.` wildcard search via DFS |
| `0225_implement_stack_using_queues.rs` | LIFO stack built on a single queue by rotating |
| `0232_implement_queue_using_stacks.rs` | FIFO queue built from two stacks, amortized O(1) |
| `0239_sliding_window_maximum_monotonic_deque.rs` | Sliding window maximum with a monotonic deque |
| `0284_peeking_iterator.rs` | Generic iterator adapter that adds peek |
| `0295_find_median_from_data_stream.rs` | Running median kept in two heaps |
| `0307_range_sum_query_mutable_segment_tree.rs` | Iterative bottom-up segment tree for point updates and range sums |
| `0380_insert_delete_getrandom_o1.rs` | Set with O(1) insert, delete and uniform get_random (Vec + HashMap + xorshift) |
| `0460_lfu_cache.rs` | LFU cache that breaks frequency ties by LRU |
| `0547_number_of_provinces_union_find.rs` | Reusable DSU (path halving, union by size) applied to counting provinces |
| `0622_design_circular_queue.rs` | Fixed-capacity ring buffer queue |
| `0641_design_circular_deque.rs` | Fixed-capacity ring buffer deque |
| `0703_kth_largest_element_in_a_stream.rs` | k-th largest in a stream using a size-k min-heap |
| `0705_design_hashset.rs` | Integer hash set with separate chaining |
| `0706_design_hashmap_open_addressing.rs` | Generic hash map with linear probing, tombstones and resizing |
| `0707_design_linked_list.rs` | Singly linked list with indexed insert and delete, and an iterative Drop |
| `0895_maximum_frequency_stack.rs` | Stack that pops the most frequent element, using per-frequency stacks |
| `0981_time_based_key_value_store.rs` | Versioned key-value store with binary search by timestamp |
| `1206_design_skiplist.rs` | Arena-based skiplist with duplicates, erase and slot recycling |
| `1352_product_of_the_last_k_numbers.rs` | Prefix products that reset on zero |
| `avl_tree.rs` | Self-balancing AVL ordered set with rotations and k-th smallest |
| `binary_heap_generic.rs` | Generic min-heap with O(n) heapify, plus heapsort |
| `binary_search_tree.rs` | Unbalanced BST with insert, contains, successor-based delete and in-order walk |
| `bloom_filter.rs` | Bloom filter sized from n and p, using double hashing |
| `dynamic_array_from_scratch.rs` | Minimal `Vec<T>` on raw `std::alloc`, with insert, remove and Drop |
| `fenwick_tree_binary_indexed_tree.rs` | Fenwick tree with O(n) build, range sums and lower_bound |
| `lazy_segment_tree_range_add_range_sum.rs` | Segment tree with lazy propagation for range add and range sum |
| `sparse_table_rmq.rs` | Sparse table for O(1) static range-minimum queries |
