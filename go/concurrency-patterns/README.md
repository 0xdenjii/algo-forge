# Go Concurrency Patterns

Each file is a standalone program (tagged `//go:build ignore`, so they can share a folder). Run one with `go run <file>.go`.

## LeetCode concurrency problems
- `1114_print_in_order.go`: Print in Order, using closed channels as one-shot signals.
- `1115_print_foobar_alternately.go`: Print FooBar Alternately, passing a token between two goroutines.
- `1116_print_zero_even_odd.go`: Print Zero Even Odd, where the zero goroutine dispatches to the odd and even goroutines.
- `1117_building_h2o.go`: Building H2O, using semaphores and a cyclic barrier of 3.
- `1188_bounded_blocking_queue.go`: Bounded Blocking Queue, a ring buffer with a mutex and two condition variables.
- `1195_fizz_buzz_multithreaded.go`: Fizz Buzz Multithreaded, with a dispatcher, per-worker channels and acks.
- `1226_dining_philosophers.go`: The Dining Philosophers, avoiding deadlock by ordering fork locks.
- `1242_web_crawler_multithreaded.go`: Web Crawler Multithreaded, a concurrent traversal with a visited set and a WaitGroup.
- `1279_traffic_light_controlled_intersection.go`: Traffic Light Controlled Intersection, with a mutex and a green-road state.

## Classic patterns and problems
- `worker_pool.go`: A fixed pool of workers that returns results in input order.
- `pipeline.go`: A multi-stage channel pipeline (generate, map, filter) with done cancellation.
- `fan_in_merge_channels.go`: Merges N channels into one (generic fan-in).
- `fan_out_distribute_work.go`: One producer and many consumers, each item handled exactly once.
- `producer_consumer.go`: Multiple producers and consumers over a bounded buffered channel.
- `semaphore_bounded_parallelism.go`: A counting semaphore that limits concurrency to K, with the peak checked.
- `token_bucket_rate_limiter.go`: A token bucket limiter that refills lazily, tested with an injected clock.
- `sliding_window_rate_limiter.go`: A per-key sliding-window-log rate limiter.
- `context_cancellation.go`: An errgroup-style group where the first error cancels the other tasks.
- `timeout_with_select.go`: Wraps a call with a context timeout without leaking the goroutine.
- `or_done_channel.go`: The or-done wrapper, which reads a channel while respecting cancellation.
- `or_channel_first_signal.go`: A recursive or-channel that closes when any input closes.
- `tee_channel.go`: Duplicates a stream to two consumers.
- `bridge_channel.go`: Flattens a channel of channels into one stream.
- `heartbeat.go`: A worker sends heartbeats and a supervisor detects a stall.
- `pub_sub_broker.go`: An in-memory topic broker with subscribe, unsubscribe and close.
- `cyclic_barrier.go`: A reusable barrier built on a cond and a generation counter.
- `readers_writers_lock.go`: A hand-rolled writer-preferring RW lock with invariant checks.
- `sleeping_barber.go`: Dijkstra's sleeping barber, with a buffered channel as the waiting room.
- `parallel_merge_sort.go`: Merge sort with depth-limited parallel recursion.
- `parallel_sum_chunks.go`: Sums data in parallel chunks, one partial-sum slot per worker.
- `map_reduce_word_count.go`: A concurrent MapReduce word count.
- `singleflight_dedup.go`: Duplicate-call suppression, so there is one execution per key in flight.
- `circuit_breaker.go`: A circuit breaker state machine (closed, open, half-open).
- `batching_with_ticker.go`: Flushes a batch on size or on max wait time.
- `concurrent_prime_sieve.go`: A daisy-chain prime sieve with one filter goroutine per prime.
- `concurrent_safe_cache_ttl.go`: A generic TTL cache with a background janitor goroutine.
- `once_lazy_initialization.go`: A thread-safe singleton using `sync.Once` and `sync.OnceValue`.
- `bank_transfer_deadlock_free.go`: Concurrent account transfers kept deadlock-free by lock ordering.
- `ordered_concurrent_results.go`: A concurrent map that emits results in input order using futures.
- `retry_with_backoff.go`: Context-aware retry with exponential backoff and jitter.
