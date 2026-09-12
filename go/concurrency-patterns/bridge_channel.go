//go:build ignore

// Bridge Channel (flatten a channel of channels)
// Statement: Given a channel that yields channels, expose a single channel that
// yields every value from each inner channel in sequence.
// Approach: outer loop receives the next inner channel, inner loop forwards its
// values until it closes; all sends select on done.
// Time: O(total values). Space: O(1).
package main

import "fmt"

func bridge[T any](done <-chan struct{}, chanStream <-chan (<-chan T)) <-chan T {
	out := make(chan T)
	go func() {
		defer close(out)
		for {
			var stream <-chan T
			select {
			case s, ok := <-chanStream:
				if !ok {
					return
				}
				stream = s
			case <-done:
				return
			}
			for v := range stream {
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

func genChans() <-chan (<-chan int) {
	cs := make(chan (<-chan int))
	go func() {
		defer close(cs)
		for i := 0; i < 4; i++ {
			c := make(chan int, 3)
			for j := 0; j < 3; j++ {
				c <- i*10 + j
			}
			close(c)
			cs <- c
		}
	}()
	return cs
}

func main() {
	done := make(chan struct{})
	defer close(done)
	for v := range bridge(done, genChans()) {
		fmt.Print(v, " ")
	}
	fmt.Println()
}
