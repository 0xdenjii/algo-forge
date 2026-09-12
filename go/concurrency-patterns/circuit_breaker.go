//go:build ignore

// Circuit Breaker
// Statement: Wrap calls to an unreliable dependency: after N consecutive failures stop
// calling it (open) for a cool-down, then allow one trial call (half-open) that
// closes the circuit on success or re-opens it on failure.
// Approach: mutex-guarded state machine Closed -> Open -> HalfOpen with an injected
// clock for deterministic testing.
// Time: O(1) per call. Space: O(1).
package main

import (
	"errors"
	"fmt"
	"sync"
	"time"
)

type State int

const (
	Closed State = iota
	Open
	HalfOpen
)

func (s State) String() string { return [...]string{"closed", "open", "half-open"}[s] }

var ErrOpen = errors.New("circuit open")

type Breaker struct {
	mu        sync.Mutex
	state     State
	fails     int
	threshold int
	cooldown  time.Duration
	openedAt  time.Time
	now       func() time.Time
}

func (b *Breaker) Call(fn func() error) error {
	b.mu.Lock()
	if b.state == Open {
		if b.now().Sub(b.openedAt) < b.cooldown {
			b.mu.Unlock()
			return ErrOpen
		}
		b.state = HalfOpen
	} else if b.state == HalfOpen {
		b.mu.Unlock()
		return ErrOpen // only one trial in flight
	}
	b.mu.Unlock()

	err := fn()

	b.mu.Lock()
	defer b.mu.Unlock()
	if err != nil {
		b.fails++
		if b.state == HalfOpen || b.fails >= b.threshold {
			b.state, b.openedAt = Open, b.now()
		}
		return err
	}
	b.state, b.fails = Closed, 0
	return nil
}

func main() {
	clock := time.Unix(0, 0)
	b := &Breaker{threshold: 3, cooldown: 5 * time.Second, now: func() time.Time { return clock }}
	fail := func() error { return errors.New("boom") }
	ok := func() error { return nil }
	step := func(label string, fn func() error) {
		err := b.Call(fn)
		fmt.Printf("%-18s err=%-12v state=%v\n", label, err, b.state)
	}
	step("fail 1", fail)
	step("fail 2", fail)
	step("fail 3 -> open", fail)
	step("rejected", ok)
	clock = clock.Add(6 * time.Second)
	step("trial fails", fail)
	clock = clock.Add(6 * time.Second)
	step("trial succeeds", ok)
	step("normal", ok)
}
