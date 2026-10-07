//go:build ignore

// Counting Semaphore / Bounded Parallelism
// Statement: Run N tasks concurrently but never more than K at the same time; prove
// the bound by tracking the peak number of simultaneously running tasks.
// Approach: buffered channel of capacity K as a semaphore (send = acquire,
// receive = release); atomic counters measure concurrency.
// Time: O(N/K * task). Space: O(K).
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type Semaphore chan struct{}

func (s Semaphore) Acquire() { s <- struct{}{} }
func (s Semaphore) Release() { <-s }

func main() {
	const N, K = 30, 4
	sem := make(Semaphore, K)
	var running, peak atomic.Int32
	var wg sync.WaitGroup
	for i := 0; i < N; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			sem.Acquire()
			defer sem.Release()
			cur := running.Add(1)
			for {
				p := peak.Load()
				if cur <= p || peak.CompareAndSwap(p, cur) {
					break
				}
			}
			time.Sleep(2 * time.Millisecond)
			running.Add(-1)
		}()
	}
	wg.Wait()
	fmt.Println("peak concurrency:", peak.Load(), "limit:", K)
}
