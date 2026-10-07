//go:build ignore

// Pipeline (generator -> square -> filter -> sink)
// Statement: Build a multi-stage pipeline where each stage is a goroutine connected by
// channels, and every stage stops cleanly when the done channel is closed.
// Approach: each stage takes <-chan in, returns <-chan out, closes out on exit, and
// selects on done for every send so cancellation never leaks goroutines.
// Time: O(n). Space: O(stages).
package main

import "fmt"

func gen(done <-chan struct{}, nums ...int) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for _, n := range nums {
			select {
			case out <- n:
			case <-done:
				return
			}
		}
	}()
	return out
}

func mapStage(done <-chan struct{}, in <-chan int, f func(int) int) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for v := range in {
			select {
			case out <- f(v):
			case <-done:
				return
			}
		}
	}()
	return out
}

func filter(done <-chan struct{}, in <-chan int, keep func(int) bool) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for v := range in {
			if !keep(v) {
				continue
			}
			select {
			case out <- v:
			case <-done:
				return
			}
		}
	}()
	return out
}

func main() {
	done := make(chan struct{})
	defer close(done)
	sq := mapStage(done, gen(done, 1, 2, 3, 4, 5, 6, 7, 8), func(x int) int { return x * x })
	for v := range filter(done, sq, func(x int) bool { return x%2 == 0 }) {
		fmt.Print(v, " ")
	}
	fmt.Println()
}
