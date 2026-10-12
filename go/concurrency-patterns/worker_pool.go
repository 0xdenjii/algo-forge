//go:build ignore

// Worker Pool
// Statement: Process a stream of jobs with a fixed number of workers and collect all
// results, returning them in input order.
// Approach: jobs channel fanned out to k workers; results carry the job index so the
// collector can place them; a WaitGroup closes the results channel.
// Time: O(n * cost / k) wall clock. Space: O(n) results.
package main

import (
	"fmt"
	"sync"
)

type job struct{ idx, val int }
type result struct{ idx, out int }

func pool(inputs []int, workers int, f func(int) int) []int {
	jobs := make(chan job)
	results := make(chan result)
	var wg sync.WaitGroup
	for w := 0; w < workers; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := range jobs {
				results <- result{j.idx, f(j.val)}
			}
		}()
	}
	go func() {
		for i, v := range inputs {
			jobs <- job{i, v}
		}
		close(jobs)
	}()
	go func() { wg.Wait(); close(results) }()
	out := make([]int, len(inputs))
	for r := range results {
		out[r.idx] = r.out
	}
	return out
}

func main() {
	in := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	fmt.Println(pool(in, 3, func(x int) int { return x * x }))
}
