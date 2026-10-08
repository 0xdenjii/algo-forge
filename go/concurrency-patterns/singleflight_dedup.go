//go:build ignore

// Singleflight (duplicate call suppression)
// Statement: Many goroutines request the same expensive key at once; execute the
// underlying function only once per key in flight and share its result with all.
// Approach: mutex-guarded map key -> call{wg, val, err}; the first caller runs fn,
// later callers wait on the call's WaitGroup. Entry removed after completion.
// Time: O(1) overhead per call. Space: O(keys in flight).
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type call struct {
	wg  sync.WaitGroup
	val any
	err error
}

type Group struct {
	mu sync.Mutex
	m  map[string]*call
}

func (g *Group) Do(key string, fn func() (any, error)) (any, error, bool) {
	g.mu.Lock()
	if g.m == nil {
		g.m = map[string]*call{}
	}
	if c, ok := g.m[key]; ok {
		g.mu.Unlock()
		c.wg.Wait()
		return c.val, c.err, true
	}
	c := &call{}
	c.wg.Add(1)
	g.m[key] = c
	g.mu.Unlock()

	c.val, c.err = fn()
	c.wg.Done()

	g.mu.Lock()
	delete(g.m, key)
	g.mu.Unlock()
	return c.val, c.err, false
}

func main() {
	var g Group
	var calls atomic.Int32
	var shared atomic.Int32
	var wg sync.WaitGroup
	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			v, _, sh := g.Do("user:42", func() (any, error) {
				calls.Add(1)
				time.Sleep(20 * time.Millisecond)
				return "alice", nil
			})
			if sh {
				shared.Add(1)
			}
			_ = v
		}()
	}
	wg.Wait()
	fmt.Println("underlying calls:", calls.Load(), "shared results:", shared.Load())
}
