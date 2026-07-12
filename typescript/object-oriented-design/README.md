# Object-Oriented Design (TypeScript)

These are self-contained TypeScript solutions to "design a class" problems and low-level-design and design-pattern exercises. Each file has a header with the problem, approach and complexity, followed by a runnable demo. To run one, use `npx tsx <file>.ts`.

## LeetCode design problems

| File | Description |
| --- | --- |
| `0146_lru_cache.ts` | LRU cache with O(1) get and put. It relies on the insertion order of `Map`. |
| `0155_min_stack.ts` | Stack with O(1) getMin. Each frame stores the minimum seen so far. |
| `0173_bst_iterator.ts` | In-order BST iterator that uses O(h) stack memory. |
| `0208_implement_trie.ts` | Prefix tree with insert, search and startsWith. |
| `0211_add_and_search_words.ts` | Trie that supports the `.` wildcard in search. |
| `0225_stack_using_queues.ts` | LIFO stack built on one queue by rotating it after each push. |
| `0232_queue_using_stacks.ts` | FIFO queue built on two stacks, with amortized O(1) operations. |
| `0284_peeking_iterator.ts` | Generic iterator wrapper that adds peek() through one cached look-ahead element. |
| `0295_find_median_from_data_stream.ts` | Running median kept in two heaps. |
| `0297_serialize_deserialize_binary_tree.ts` | Binary-tree codec that writes a preorder string with null markers. |
| `0307_range_sum_query_mutable.ts` | Range sums with point updates, using a Fenwick tree. |
| `0341_flatten_nested_list_iterator.ts` | Lazy iterator over nested lists, driven by a stack of iterators. |
| `0348_design_tic_tac_toe.ts` | n x n tic-tac-toe with an O(1) win check through signed row, column and diagonal counters. |
| `0353_design_snake_game.ts` | Snake game that tracks the body in a deque and occupied cells in a set. |
| `0355_design_twitter.ts` | Twitter-style news feed that merges the 10 most recent tweets from followed users. |
| `0359_logger_rate_limiter.ts` | Suppresses a repeated message for 10 seconds. |
| `0362_design_hit_counter.ts` | Counts hits in the last 300 seconds with a circular buffer. |
| `0380_insert_delete_getrandom.ts` | Set with O(1) insert, remove and getRandom. Removal swaps the element with the last one. |
| `0460_lfu_cache.ts` | LFU cache with O(1) operations. Keys sit in frequency buckets, and ties go to the least recently used key. |
| `0622_design_circular_queue.ts` | Fixed-size ring-buffer queue. |
| `0641_design_circular_deque.ts` | Fixed-size ring-buffer deque. |
| `0705_design_hashset.ts` | Hash set that uses separate chaining. |
| `0706_design_hashmap.ts` | Hash map that uses separate chaining and resizes itself. |
| `0729_my_calendar_i.ts` | Books an event only if it overlaps no existing booking. Bookings stay sorted and are found by binary search. |
| `0731_my_calendar_ii.ts` | Calendar that rejects any booking causing a triple overlap. |
| `0901_online_stock_span.ts` | Stock span computed with a monotonic stack. |
| `0981_time_based_key_value_store.ts` | Versioned key-value store that answers lookups by timestamp. |
| `1146_snapshot_array.ts` | Array with snapshots. Each index keeps its own history, searched by binary search. |
| `1352_product_of_last_k_numbers.ts` | Product of the last k numbers, from prefix products that reset when a zero arrives. |
| `1472_design_browser_history.ts` | Browser back and forward navigation in which a new visit clears the forward history. |
| `1603_design_parking_system.ts` | Parking with fixed slot counts for each car size. |

## Low-level design and design patterns

| File | Description |
| --- | --- |
| `lld_parking_lot.ts` | Multi-level parking lot with a vehicle hierarchy, tickets and a pluggable pricing strategy. |
| `lld_vending_machine_state_pattern.ts` | Vending machine built on the State pattern. It handles coins, selection, change, sold-out items and refunds. |
| `lld_elevator_system.ts` | Multi-car elevator controller. It dispatches calls by cost and schedules each car with LOOK. |
| `lld_token_bucket_rate_limiter.ts` | Rate limiter with token-bucket and sliding-window-log strategies and an injectable clock. |
| `lld_library_management.ts` | Library with loans, loan limits, overdue fines and reservations. Expected failures are returned as Result types. |
| `pattern_command_text_editor_undo_redo.ts` | Text editor with undo and redo built on the Command pattern. Replace is a composite of a delete and an insert. |
| `pattern_observer_typed_event_emitter.ts` | Observer pattern implemented as a type-safe event emitter with on, once and off. |
