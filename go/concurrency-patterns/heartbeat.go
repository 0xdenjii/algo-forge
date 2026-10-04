//go:build ignore

// Heartbeat
// Statement: A long-running worker must signal liveness at a fixed interval, so a
// supervisor can detect when it stalls and stop waiting on it.
// Approach: worker selects over a ticker (non-blocking heartbeat send), work output,
// and done; the supervisor resets a timeout each time a heartbeat arrives.
// Time: O(duration/interval). Space: O(1).
package main

import (
	"fmt"
	"time"
)

func worker(done <-chan struct{}, pulse time.Duration, stallAfter int) (<-chan struct{}, <-chan int) {
	hb := make(chan struct{}, 1)
	results := make(chan int)
	go func() {
		defer close(hb)
		defer close(results)
		tick := time.NewTicker(pulse)
		defer tick.Stop()
		work := time.NewTicker(pulse / 2)
		defer work.Stop()
		for i := 0; ; i++ {
			if i == stallAfter {
				<-done // simulate a hung worker: stop beating
				return
			}
			select {
			case <-done:
				return
			case <-tick.C:
				select {
				case hb <- struct{}{}:
				default:
				}
			case <-work.C:
				select {
				case results <- i:
				case <-done:
					return
				}
			}
		}
	}()
	return hb, results
}

func main() {
	done := make(chan struct{})
	defer close(done)
	const pulse = 10 * time.Millisecond
	hb, results := worker(done, pulse, 12)
	beats, got := 0, 0
	timeout := time.NewTimer(2 * pulse)
	for {
		select {
		case <-hb:
			beats++
			timeout.Reset(2 * pulse)
		case _, ok := <-results:
			if ok {
				got++
			}
		case <-timeout.C:
			fmt.Printf("worker unhealthy: no heartbeat (beats=%d, results=%d)\n", beats, got)
			return
		}
	}
}
