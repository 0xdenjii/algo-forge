//go:build ignore

// LeetCode 1117 - Building H2O
// Statement: Hydrogen and oxygen goroutines arrive in any order. Each molecule must
// release exactly two H and one O before atoms of the next molecule are released.
// Approach: semaphores (buffered channels) admit at most 2 H and 1 O; a reusable
// cyclic barrier of 3 groups them; a second barrier pass frees the slots together.
// Time: O(1) per atom. Space: O(1).
package main

import (
	"fmt"
	"strings"
	"sync"
)

type barrier struct {
	mu       sync.Mutex
	cond     *sync.Cond
	n, count int
	gen      int
}

func newBarrier(n int) *barrier { b := &barrier{n: n}; b.cond = sync.NewCond(&b.mu); return b }

func (b *barrier) await() {
	b.mu.Lock()
	g := b.gen
	b.count++
	if b.count == b.n {
		b.count = 0
		b.gen++
		b.cond.Broadcast()
	} else {
		for g == b.gen {
			b.cond.Wait()
		}
	}
	b.mu.Unlock()
}

type H2O struct {
	hSem, oSem chan struct{}
	bar        *barrier
}

func NewH2O() *H2O { return &H2O{make(chan struct{}, 2), make(chan struct{}, 1), newBarrier(3)} }

func (h *H2O) Hydrogen(release func()) {
	h.hSem <- struct{}{}
	h.bar.await()
	release()
	h.bar.await()
	<-h.hSem
}

func (h *H2O) Oxygen(release func()) {
	h.oSem <- struct{}{}
	h.bar.await()
	release()
	h.bar.await()
	<-h.oSem
}

func main() {
	input := "OOHHHHHOHHHO" // 4 O, 8 H
	w := NewH2O()
	var mu sync.Mutex
	var sb strings.Builder
	rel := func(s string) func() { return func() { mu.Lock(); sb.WriteString(s); mu.Unlock() } }
	var wg sync.WaitGroup
	for _, c := range input {
		wg.Add(1)
		if c == 'H' {
			go func() { defer wg.Done(); w.Hydrogen(rel("H")) }()
		} else {
			go func() { defer wg.Done(); w.Oxygen(rel("O")) }()
		}
	}
	wg.Wait()
	out := sb.String()
	ok := true
	for i := 0; i < len(out); i += 3 {
		if strings.Count(out[i:i+3], "H") != 2 {
			ok = false
		}
	}
	fmt.Println(out, "valid:", ok)
}
