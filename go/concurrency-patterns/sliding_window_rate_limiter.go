//go:build ignore

// Sliding Window Log Rate Limiter (per key)
// Statement: Allow at most `limit` requests per key in any rolling window of length
// `window`; must be safe for concurrent callers.
// Approach: per-key queue of timestamps; on each request drop timestamps older than
// now-window, then accept if fewer than limit remain. Mutex guards the map.
// Time: O(1) amortized. Space: O(keys * limit).
package main

import (
	"fmt"
	"sync"
	"time"
)

type Limiter struct {
	mu     sync.Mutex
	limit  int
	window time.Duration
	log    map[string][]time.Time
}

func NewLimiter(limit int, window time.Duration) *Limiter {
	return &Limiter{limit: limit, window: window, log: map[string][]time.Time{}}
}

func (l *Limiter) Allow(key string, now time.Time) bool {
	l.mu.Lock()
	defer l.mu.Unlock()
	q := l.log[key]
	i := 0
	for i < len(q) && !q[i].After(now.Add(-l.window)) {
		i++
	}
	q = q[i:]
	ok := len(q) < l.limit
	if ok {
		q = append(q, now)
	}
	l.log[key] = q
	return ok
}

func main() {
	l := NewLimiter(3, time.Second)
	t0 := time.Unix(100, 0)
	at := func(ms int) time.Time { return t0.Add(time.Duration(ms) * time.Millisecond) }
	for _, ms := range []int{0, 100, 200, 300, 999, 1000, 1101} {
		fmt.Printf("t=%4dms alice allowed=%v\n", ms, l.Allow("alice", at(ms)))
	}
	var wg sync.WaitGroup
	allowed := make(chan bool, 50)
	for i := 0; i < 50; i++ {
		wg.Add(1)
		go func() { defer wg.Done(); allowed <- l.Allow("bob", t0) }()
	}
	wg.Wait()
	close(allowed)
	n := 0
	for a := range allowed {
		if a {
			n++
		}
	}
	fmt.Println("bob concurrent allowed:", n)
}
