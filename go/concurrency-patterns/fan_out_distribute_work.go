//go:build ignore

// Fan-Out (one producer, many consumers)
// Statement: Distribute items from a single channel across N consumers so each item
// is handled by exactly one consumer, then report how many each handled.
// Approach: all consumers range over the same channel; Go's channel receive makes
// each item go to exactly one receiver. Per-worker counters avoid shared state.
// Time: O(n). Space: O(workers).
package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	const workers, items = 4, 40
	in := make(chan int)
	counts := make([]int, workers)
	sums := make([]int, workers)
	var wg sync.WaitGroup
	for w := 0; w < workers; w++ {
		wg.Add(1)
		go func(w int) {
			defer wg.Done()
			for v := range in {
				time.Sleep(time.Millisecond)
				counts[w]++
				sums[w] += v
			}
		}(w)
	}
	for i := 1; i <= items; i++ {
		in <- i
	}
	close(in)
	wg.Wait()
	total, n := 0, 0
	for w := range counts {
		total += sums[w]
		n += counts[w]
	}
	fmt.Println("per-worker counts:", counts, "items:", n, "sum:", total, "expected:", items*(items+1)/2)
}
