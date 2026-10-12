//go:build ignore

// Timeout with select
// Statement: Call a possibly slow function but give up after a deadline, returning a
// timeout error; the slow goroutine must not leak or block forever.
// Approach: run the call in a goroutine that writes to a buffered(1) channel, and
// select between that channel and context deadline.
// Time: O(min(call, timeout)). Space: O(1).
package main

import (
	"context"
	"errors"
	"fmt"
	"time"
)

func callWithTimeout[T any](timeout time.Duration, f func() T) (T, error) {
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()
	ch := make(chan T, 1) // buffered so the worker can always finish
	go func() { ch <- f() }()
	select {
	case v := <-ch:
		return v, nil
	case <-ctx.Done():
		var zero T
		return zero, ctx.Err()
	}
}

func slow(d time.Duration, v string) func() string {
	return func() string { time.Sleep(d); return v }
}

func main() {
	v, err := callWithTimeout(50*time.Millisecond, slow(10*time.Millisecond, "fast result"))
	fmt.Printf("%q %v\n", v, err)
	v, err = callWithTimeout(50*time.Millisecond, slow(200*time.Millisecond, "slow result"))
	fmt.Printf("%q %v deadline=%v\n", v, err, errors.Is(err, context.DeadlineExceeded))
}
