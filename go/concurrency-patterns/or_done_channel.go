//go:build ignore

// Or-Done Channel
// Statement: Read from a channel you don't control while also respecting a done
// signal, so the reader stops as soon as either the source closes or done closes.
// Approach: wrap the source in a goroutine that selects on done for both the receive
// and the forward send; the wrapper closes its output on exit.
// Time: O(values read). Space: O(1).
package main

import (
	"fmt"
	"time"
)

func orDone[T any](done <-chan struct{}, c <-chan T) <-chan T {
	out := make(chan T)
	go func() {
		defer close(out)
		for {
			select {
			case <-done:
				return
			case v, ok := <-c:
				if !ok {
					return
				}
				select {
				case out <- v:
				case <-done:
					return
				}
			}
		}
	}()
	return out
}

func infinite() <-chan int {
	c := make(chan int)
	go func() {
		for i := 0; ; i++ {
			c <- i
			time.Sleep(time.Millisecond)
		}
	}()
	return c
}

func main() {
	done := make(chan struct{})
	go func() { time.Sleep(15 * time.Millisecond); close(done) }()
	n := 0
	for range orDone(done, infinite()) {
		n++
	}
	fmt.Println("stopped after reading", n, "values from an infinite source")
}
