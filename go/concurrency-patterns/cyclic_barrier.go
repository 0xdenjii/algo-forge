//go:build ignore

// Cyclic Barrier
// Statement: N goroutines run in phases; none may start phase k+1 until all have
// finished phase k. The barrier must be reusable across phases.
// Approach: mutex + cond with a generation counter; the last arrival bumps the
// generation and broadcasts, waiters loop until generation changes.
// Time: O(N) per phase. Space: O(1).
package main

import (
	"fmt"
	"sync"
)

type Barrier struct {
	mu         sync.Mutex
	cond       *sync.Cond
	n, waiting int
	generation int
}

func NewBarrier(n int) *Barrier { b := &Barrier{n: n}; b.cond = sync.NewCond(&b.mu); return b }

func (b *Barrier) Await() {
	b.mu.Lock()
	defer b.mu.Unlock()
	gen := b.generation
	b.waiting++
	if b.waiting == b.n {
		b.waiting = 0
		b.generation++
		b.cond.Broadcast()
		return
	}
	for gen == b.generation {
		b.cond.Wait()
	}
}

func main() {
	const N, phases = 5, 3
	b := NewBarrier(N)
	var mu sync.Mutex
	var log []int // phase of each completed step, in completion order
	var wg sync.WaitGroup
	for g := 0; g < N; g++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for p := 0; p < phases; p++ {
				mu.Lock()
				log = append(log, p)
				mu.Unlock()
				b.Await()
			}
		}()
	}
	wg.Wait()
	ok := true
	for i := 1; i < len(log); i++ {
		if log[i] < log[i-1] {
			ok = false
		}
	}
	fmt.Println("log:", log, "phases never interleave:", ok)
}
