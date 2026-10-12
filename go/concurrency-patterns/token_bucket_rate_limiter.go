//go:build ignore

// Token Bucket Rate Limiter
// Statement: Allow at most `rate` events per second on average with bursts up to
// `capacity`; Allow() returns whether an event may proceed right now.
// Approach: lazily refill tokens from elapsed time under a mutex (no background
// goroutine); consume one token per allowed event.
// Time: O(1) per call. Space: O(1).
package main

import (
	"fmt"
	"sync"
	"time"
)

type TokenBucket struct {
	mu       sync.Mutex
	capacity float64
	tokens   float64
	rate     float64 // tokens per second
	last     time.Time
	now      func() time.Time
}

func NewTokenBucket(rate, capacity float64, now func() time.Time) *TokenBucket {
	return &TokenBucket{capacity: capacity, tokens: capacity, rate: rate, last: now(), now: now}
}

func (b *TokenBucket) Allow() bool {
	b.mu.Lock()
	defer b.mu.Unlock()
	t := b.now()
	b.tokens += t.Sub(b.last).Seconds() * b.rate
	if b.tokens > b.capacity {
		b.tokens = b.capacity
	}
	b.last = t
	if b.tokens >= 1 {
		b.tokens--
		return true
	}
	return false
}

func main() {
	clock := time.Unix(0, 0)
	b := NewTokenBucket(2, 3, func() time.Time { return clock }) // 2/s, burst 3
	var res []bool
	for i := 0; i < 5; i++ {
		res = append(res, b.Allow())
	}
	fmt.Println("burst at t=0:   ", res) // true true true false false
	clock = clock.Add(time.Second)
	res = res[:0]
	for i := 0; i < 3; i++ {
		res = append(res, b.Allow())
	}
	fmt.Println("after 1 second: ", res) // true true false
}
