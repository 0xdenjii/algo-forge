//go:build ignore

// Ordered Concurrent Map (preserve input order while streaming)
// Statement: Apply a slow function to a stream concurrently, but emit results in the
// original input order as soon as each prefix is ready.
// Approach: for each input, create a per-item future channel and push it onto an
// ordered queue channel; a semaphore bounds parallelism; the consumer reads futures
// in order.
// Time: O(n * cost / k). Space: O(k) in-flight futures.
package main

import (
	"fmt"
	"math/rand"
	"time"
)

func orderedMap[T, R any](in <-chan T, k int, f func(T) R) <-chan R {
	futures := make(chan chan R, k)
	out := make(chan R)
	go func() {
		defer close(futures)
		for v := range in {
			fut := make(chan R, 1)
			futures <- fut // blocks when k items in flight
			go func(v T) { fut <- f(v) }(v)
		}
	}()
	go func() {
		defer close(out)
		for fut := range futures {
			out <- <-fut
		}
	}()
	return out
}

func main() {
	in := make(chan int)
	go func() {
		defer close(in)
		for i := 1; i <= 10; i++ {
			in <- i
		}
	}()
	r := rand.New(rand.NewSource(7))
	delays := make([]time.Duration, 11)
	for i := range delays {
		delays[i] = time.Duration(r.Intn(20)) * time.Millisecond
	}
	for v := range orderedMap(in, 4, func(x int) string {
		time.Sleep(delays[x])
		return fmt.Sprintf("%d^2=%d", x, x*x)
	}) {
		fmt.Println(v)
	}
}
