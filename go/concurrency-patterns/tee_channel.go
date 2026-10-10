//go:build ignore

// Tee Channel
// Statement: Split one input channel into two output channels so each value is
// delivered to both consumers (like the Unix tee command).
// Approach: for each value, send to both outputs in a select loop, nil-ing out a
// channel variable once it has received so the other send can proceed.
// Time: O(n). Space: O(1).
package main

import (
	"fmt"
	"sync"
)

func tee[T any](done <-chan struct{}, in <-chan T) (<-chan T, <-chan T) {
	o1, o2 := make(chan T), make(chan T)
	go func() {
		defer close(o1)
		defer close(o2)
		for v := range in {
			a, b := o1, o2
			for i := 0; i < 2; i++ {
				select {
				case <-done:
					return
				case a <- v:
					a = nil
				case b <- v:
					b = nil
				}
			}
		}
	}()
	return o1, o2
}

func main() {
	done := make(chan struct{})
	defer close(done)
	src := make(chan int)
	go func() {
		defer close(src)
		for i := 1; i <= 5; i++ {
			src <- i
		}
	}()
	a, b := tee(done, src)
	var sa, sb []int
	var wg sync.WaitGroup
	wg.Add(2)
	go func() { defer wg.Done(); for v := range a { sa = append(sa, v) } }()
	go func() { defer wg.Done(); for v := range b { sb = append(sb, v*10) } }()
	wg.Wait()
	fmt.Println("consumer A:", sa)
	fmt.Println("consumer B:", sb)
}
