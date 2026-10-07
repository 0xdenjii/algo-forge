//go:build ignore

// Retry with Exponential Backoff and Jitter (context-aware)
// Statement: Retry a failing operation up to N attempts, waiting base*2^i (capped,
// with jitter) between attempts, and abort immediately if the context is cancelled.
// Approach: loop attempts; sleep via select on time.After and ctx.Done so
// cancellation interrupts the wait; return the last error wrapped.
// Time: O(attempts). Space: O(1).
package main

import (
	"context"
	"errors"
	"fmt"
	"math/rand"
	"time"
)

func Retry(ctx context.Context, attempts int, base, maxDelay time.Duration, op func(int) error) error {
	var err error
	for i := 0; i < attempts; i++ {
		if err = op(i); err == nil {
			return nil
		}
		if i == attempts-1 {
			break
		}
		d := base << i
		if d > maxDelay {
			d = maxDelay
		}
		d = d/2 + time.Duration(rand.Int63n(int64(d/2)+1)) // equal jitter
		select {
		case <-time.After(d):
		case <-ctx.Done():
			return fmt.Errorf("retry aborted: %w (last: %v)", ctx.Err(), err)
		}
	}
	return fmt.Errorf("after %d attempts: %w", attempts, err)
}

var errFlaky = errors.New("flaky")

func main() {
	ctx := context.Background()
	err := Retry(ctx, 5, 2*time.Millisecond, 20*time.Millisecond, func(i int) error {
		fmt.Println("attempt", i)
		if i < 2 {
			return errFlaky
		}
		return nil
	})
	fmt.Println("result:", err)

	err = Retry(ctx, 3, time.Millisecond, 5*time.Millisecond, func(int) error { return errFlaky })
	fmt.Println("result:", err, "is flaky:", errors.Is(err, errFlaky))

	cctx, cancel := context.WithTimeout(ctx, 15*time.Millisecond)
	defer cancel()
	err = Retry(cctx, 10, 50*time.Millisecond, time.Second, func(int) error { return errFlaky })
	fmt.Println("result:", err)
}
