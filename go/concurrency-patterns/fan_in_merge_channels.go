//go:build ignore

// Fan-In (merge N channels)
// Statement: Given several input channels, produce one output channel that yields
// every value from all inputs and closes once all inputs are closed.
// Approach: one forwarding goroutine per input plus a WaitGroup; a closer goroutine
// closes the output after all forwarders finish.
// Time: O(total values). Space: O(number of inputs).
package main

import (
	"fmt"
	"sort"
	"sync"
)

func merge[T any](done <-chan struct{}, cs ...<-chan T) <-chan T {
	out := make(chan T)
	var wg sync.WaitGroup
	wg.Add(len(cs))
	for _, c := range cs {
		go func(c <-chan T) {
			defer wg.Done()
			for v := range c {
				select {
				case out <- v:
				case <-done:
					return
				}
			}
		}(c)
	}
	go func() { wg.Wait(); close(out) }()
	return out
}

func source(vals ...int) <-chan int {
	c := make(chan int)
	go func() {
		defer close(c)
		for _, v := range vals {
			c <- v
		}
	}()
	return c
}

func main() {
	done := make(chan struct{})
	defer close(done)
	var got []int
	for v := range merge(done, source(1, 4, 7), source(2, 5, 8), source(3, 6, 9)) {
		got = append(got, v)
	}
	sort.Ints(got)
	fmt.Println(got)
}
