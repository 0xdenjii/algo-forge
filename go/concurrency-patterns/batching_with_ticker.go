//go:build ignore

// Batching by Size or Time
// Statement: Group incoming events into batches, flushing when a batch reaches max
// size or when a max wait time elapses since the batch started, whichever is first.
// Approach: single goroutine selects over input and a timer that is armed when the
// first item of a batch arrives; flush on size, timer, or input close.
// Time: O(n). Space: O(batch size).
package main

import (
	"fmt"
	"time"
)

func batcher[T any](in <-chan T, maxSize int, maxWait time.Duration) <-chan []T {
	out := make(chan []T)
	go func() {
		defer close(out)
		var batch []T
		var timer *time.Timer
		var timeout <-chan time.Time
		flush := func() {
			if len(batch) > 0 {
				out <- batch
				batch = nil
			}
			if timer != nil {
				timer.Stop()
			}
			timeout = nil
		}
		for {
			select {
			case v, ok := <-in:
				if !ok {
					flush()
					return
				}
				if len(batch) == 0 {
					timer = time.NewTimer(maxWait)
					timeout = timer.C
				}
				batch = append(batch, v)
				if len(batch) == maxSize {
					flush()
				}
			case <-timeout:
				flush()
			}
		}
	}()
	return out
}

func main() {
	in := make(chan int)
	go func() {
		defer close(in)
		for i := 1; i <= 7; i++ { // burst: fills batches of 3
			in <- i
		}
		time.Sleep(60 * time.Millisecond) // pause: timer flushes leftover
		in <- 8
		in <- 9
	}()
	for b := range batcher(in, 3, 30*time.Millisecond) {
		fmt.Println(b)
	}
}
