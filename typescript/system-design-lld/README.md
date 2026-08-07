# TypeScript - System Design / LLD

Each file is self-contained and runs on its own (for example `npx tsx 0146_lru_cache.ts`).

| File | Description |
|---|---|
| 0146_lru_cache.ts | LRU cache with O(1) get/put using Map insertion order |
| 0155_min_stack.ts | Stack with O(1) getMin via (value, min) pairs |
| 0208_implement_trie.ts | Prefix tree with insert/search/startsWith |
| 0348_design_tic_tac_toe.ts | O(1)-per-move tic-tac-toe winner detection |
| 0353_design_snake_game.ts | Snake game with a body queue and an occupancy set |
| 0355_design_twitter.ts | Follow graph and a 10-item news feed via k-way merge |
| 0359_logger_rate_limiter.ts | Allows each message to print at most once per 10 seconds |
| 0362_design_hit_counter.ts | Counts hits in the last 300 s with circular buckets |
| 0379_design_phone_directory.ts | Hands out and releases numbers from a free stack |
| 0460_lfu_cache.ts | LFU cache with O(1) ops using frequency buckets |
| 0535_tinyurl_encode_decode.ts | URL shortener using base62 ids |
| 0622_design_circular_queue.ts | Ring-buffer FIFO queue |
| 0641_design_circular_deque.ts | Ring-buffer double-ended queue |
| 0642_search_autocomplete_system.ts | Trie-based top-3 sentence autocomplete |
| 0705_design_hashset.ts | Hash set with separate chaining |
| 0706_design_hashmap.ts | Hash map with separate chaining |
| 0981_time_based_key_value_store.ts | Versioned key-value store with binary-search lookups |
| 1146_snapshot_array.ts | Array with snapshots stored as per-index history |
| 1188_bounded_blocking_queue.ts | Async bounded blocking producer/consumer queue |
| 1396_design_underground_system.ts | Average travel time between stations |
| 1472_design_browser_history.ts | Back/forward browser history |
| 1603_design_parking_system.ts | Parking slot counters by car size |
| circuit_breaker.ts | Closed/open/half-open circuit breaker with an injectable clock |
| consistent_hashing_ring.ts | Consistent hash ring with virtual nodes |
| elevator_system.ts | Multi-car elevator controller with LOOK scheduling |
| library_management_system.ts | Loans, holds, loan limits and late fines |
| parking_lot_oop.ts | Multi-level parking lot with tickets and pricing |
| payment_strategy_and_factory.ts | Checkout using the Strategy and registry-Factory patterns |
| pub_sub_event_bus_observer.ts | Typed event bus with on/once/off and error isolation |
| rate_limiter_token_bucket.ts | Token-bucket and sliding-window-log rate limiters |
| splitwise_expense_sharing.ts | Equal/exact/percent expense splits and greedy settlement |
| vending_machine_state_pattern.ts | Vending machine built with the State pattern |
