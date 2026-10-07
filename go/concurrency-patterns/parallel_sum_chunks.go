//go:build ignore

// Parallel Sum by Chunks (data parallelism)
// Statement: Sum a large slice using all CPU cores, with no data races and no lock
// contention on the hot path.
// Approach: split into GOMAXPROCS chunks; each goroutine writes its partial sum into
// its own slot of a results slice; combine after WaitGroup.Wait.
// Time: O(n/p + p). Space: O(p).
package main

import (
	"fmt"
	"runtime"
	"sync"
)

func parallelSum(a []int64) int64 {
	p := runtime.GOMAXPROCS(0)
	if p > len(a) {
		p = max(len(a), 1)
	}
	partial := make([]int64, p)
	chunk := (len(a) + p - 1) / p
	var wg sync.WaitGroup
	for w := 0; w < p; w++ {
		lo := w * chunk
		hi := min(lo+chunk, len(a))
		if lo >= hi {
			continue
		}
		wg.Add(1)
		go func(w, lo, hi int) {
			defer wg.Done()
			var s int64
			for _, v := range a[lo:hi] {
				s += v
			}
			partial[w] = s
		}(w, lo, hi)
	}
	wg.Wait()
	var total int64
	for _, s := range partial {
		total += s
	}
	return total
}

func main() {
	n := 10_000_000
	a := make([]int64, n)
	for i := range a {
		a[i] = int64(i + 1)
	}
	fmt.Println(parallelSum(a), "expected:", int64(n)*int64(n+1)/2)
	fmt.Println(parallelSum([]int64{}), parallelSum([]int64{42}))
}
