//go:build ignore

// Producer-Consumer (multiple producers, multiple consumers)
// Statement: P producers each emit M items into a bounded buffer; C consumers drain
// it. Verify every item is consumed exactly once and nothing deadlocks.
// Approach: buffered channel as the bounded buffer; a WaitGroup on producers closes
// the channel, which ends the consumers' range loops.
// Time: O(P*M). Space: O(buffer).
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

func main() {
	const P, M, C, buf = 3, 1000, 5, 16
	ch := make(chan int, buf)
	var prod, cons sync.WaitGroup
	for p := 0; p < P; p++ {
		prod.Add(1)
		go func(p int) {
			defer prod.Done()
			for i := 0; i < M; i++ {
				ch <- p*M + i
			}
		}(p)
	}
	go func() { prod.Wait(); close(ch) }()
	var count, sum atomic.Int64
	for c := 0; c < C; c++ {
		cons.Add(1)
		go func() {
			defer cons.Done()
			for v := range ch {
				count.Add(1)
				sum.Add(int64(v))
			}
		}()
	}
	cons.Wait()
	n := int64(P * M)
	fmt.Println("consumed:", count.Load(), "sum ok:", sum.Load() == n*(n-1)/2)
}
