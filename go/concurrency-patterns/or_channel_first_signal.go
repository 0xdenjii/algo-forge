//go:build ignore

// Or-Channel (combine N done signals)
// Statement: Given any number of done channels, return a single channel that closes
// as soon as any one of them closes.
// Approach: recursive divide-and-conquer from "Concurrency in Go": handle 0/1/2
// directly, otherwise select on the first few plus or() of the rest.
// Time: O(n) goroutines total. Space: O(n).
package main

import (
	"fmt"
	"time"
)

func or(chans ...<-chan struct{}) <-chan struct{} {
	switch len(chans) {
	case 0:
		return nil
	case 1:
		return chans[0]
	}
	orDone := make(chan struct{})
	go func() {
		defer close(orDone)
		switch len(chans) {
		case 2:
			select {
			case <-chans[0]:
			case <-chans[1]:
			}
		default:
			select {
			case <-chans[0]:
			case <-chans[1]:
			case <-chans[2]:
			case <-or(append(chans[3:], orDone)...):
			}
		}
	}()
	return orDone
}

func after(d time.Duration) <-chan struct{} {
	c := make(chan struct{})
	go func() { defer close(c); time.Sleep(d) }()
	return c
}

func main() {
	start := time.Now()
	<-or(after(2*time.Second), after(time.Second), after(20*time.Millisecond), after(time.Minute), after(5*time.Second))
	fmt.Printf("done after ~%dms (expected ~20ms)\n", time.Since(start).Milliseconds())
}
