# Concurrency Problems (Rust)

Each file is a standalone program using only `std`. To run one: `rustc -O --edition 2021 <file>.rs && ./<file>`.

## LeetCode concurrency
- `1114_print_in_order.rs`: three threads print in a fixed order, using a stage counter with Mutex and Condvar.
- `1115_print_foobar_alternately.rs`: two threads alternate "foo" and "bar" using a turn flag.
- `1116_print_zero_even_odd.rs`: zero, even and odd threads interleave to print "0102...0n".
- `1117_building_h2o.rs`: releases atoms in groups of exactly two H and one O.
- `1188_bounded_blocking_queue.rs`: bounded blocking queue with not_full and not_empty condvars.
- `1195_fizz_buzz_multithreaded.rs`: four threads cooperate to print FizzBuzz in order.
- `1226_dining_philosophers.rs`: avoids deadlock by always taking the lower-numbered fork first.
- `1242_web_crawler_multithreaded.rs`: worker-pool crawler with a shared visited set and an in-flight counter to detect when it is done.
- `1279_traffic_light_controlled_intersection.rs`: one green road at a time; the lock is held while switching and crossing.
- `0146_thread_safe_lru_cache.rs`: O(1) LRU cache built on an index-linked list, behind a Mutex.
- `0204_parallel_count_primes.rs`: segmented sieve that splits Count Primes across scoped threads.

## Classic synchronization problems
- `producer_consumer_mpsc.rs`: several producers and consumers over a bounded `sync_channel`, with clean shutdown.
- `readers_writers_lock.rs`: hand-written writer-preferring RW lock with RAII guards.
- `sleeping_barber.rs`: Dijkstra's sleeping barber with a fixed number of waiting chairs.
- `cigarette_smokers.rs`: Patil's cigarette smokers problem, with an agent and three smokers.
- `bank_transfer_deadlock_free.rs`: concurrent transfers made deadlock-free by locking accounts in a global order; total money stays the same.

## Synchronizers
- `countdown_latch.rs`: CountDownLatch with wait and wait_timeout.
- `cyclic_barrier.rs`: reusable barrier that uses a generation counter.
- `counting_semaphore.rs`: counting semaphore whose RAII permits are released on drop.
- `oneshot_future_promise.rs`: one-shot promise and future; `set(self)` lets the promise be fulfilled only once.

## Lock-free and atomics
- `spsc_lock_free_ring_buffer.rs`: wait-free single-producer, single-consumer ring buffer using Acquire/Release ordering.
- `spin_lock.rs`: test-and-test-and-set spin lock with a guard.

## Patterns, LLD and parallel algorithms
- `thread_pool.rs`: fixed-size thread pool that returns results and shuts down gracefully.
- `sharded_concurrent_hashmap.rs`: hash map striped across RwLock shards, with atomic upsert.
- `token_bucket_rate_limiter.rs`: thread-safe token bucket that refills lazily.
- `delayed_task_scheduler.rs`: runs delayed tasks from a min-heap, waiting on a Condvar with a timeout.
- `pipeline_stages.rs`: generic multi-stage pipeline joined by bounded channels.
- `lazy_singleton_oncelock.rs`: global initialized exactly once through `OnceLock`.
- `parallel_merge_sort.rs`: merge sort using `thread::scope` and `split_at_mut`.
- `parallel_word_count_map_reduce.rs`: word count done as a map step per thread, then a reduce step that merges the results.
