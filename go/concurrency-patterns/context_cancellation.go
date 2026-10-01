//go:build ignore

// Context Cancellation / First Error Cancels All (errgroup-style)
// Statement: Run several tasks concurrently; if any fails, cancel the rest and return
// the first error. Otherwise return nil after all complete.
// Approach: a tiny Group wrapping context.WithCancelCause, sync.Once for the first
// error, and a WaitGroup; tasks watch ctx.Done() to stop early.
// Time: O(longest task until first failure). Space: O(tasks).
package main

import (
	"context"
	"errors"
	"fmt"
	"sync"
	"time"
)

type Group struct {
	wg     sync.WaitGroup
	once   sync.Once
	err    error
	cancel context.CancelCauseFunc
}

func WithContext(ctx context.Context) (*Group, context.Context) {
	ctx, cancel := context.WithCancelCause(ctx)
	return &Group{cancel: cancel}, ctx
}

func (g *Group) Go(f func() error) {
	g.wg.Add(1)
	go func() {
		defer g.wg.Done()
		if err := f(); err != nil {
			g.once.Do(func() { g.err = err; g.cancel(err) })
		}
	}()
}

func (g *Group) Wait() error { g.wg.Wait(); g.cancel(nil); return g.err }

func task(ctx context.Context, id int, d time.Duration, fail bool, log chan<- string) error {
	select {
	case <-time.After(d):
		if fail {
			return fmt.Errorf("task %d failed", id)
		}
		log <- fmt.Sprintf("task %d done", id)
		return nil
	case <-ctx.Done():
		log <- fmt.Sprintf("task %d cancelled", id)
		return ctx.Err()
	}
}

func main() {
	log := make(chan string, 10)
	g, ctx := WithContext(context.Background())
	g.Go(func() error { return task(ctx, 1, 10*time.Millisecond, false, log) })
	g.Go(func() error { return task(ctx, 2, 30*time.Millisecond, true, log) })
	g.Go(func() error { return task(ctx, 3, 500*time.Millisecond, false, log) })
	err := g.Wait()
	close(log)
	for l := range log {
		fmt.Println(l)
	}
	fmt.Println("first error:", err, "| is cancel:", errors.Is(err, context.Canceled))
}
