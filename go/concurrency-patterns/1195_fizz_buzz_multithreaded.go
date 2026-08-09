//go:build ignore

// LeetCode 1195 - Fizz Buzz Multithreaded
// Statement: Four goroutines print "fizz", "buzz", "fizzbuzz" and plain numbers for
// 1..n so that the combined output is the classic FizzBuzz sequence.
// Approach: each worker owns an input channel; a dispatcher walks i = 1..n, hands i
// to the right worker and waits for its ack before moving on.
// Time: O(n). Space: O(1).
package main

import (
	"fmt"
	"strings"
	"sync"
)

func fizzBuzz(n int) string {
	var sb strings.Builder
	fizz, buzz, fb, num := make(chan int), make(chan int), make(chan int), make(chan int)
	ack := make(chan struct{})
	var wg sync.WaitGroup
	worker := func(in chan int, f func(int)) {
		defer wg.Done()
		for i := range in {
			f(i)
			ack <- struct{}{}
		}
	}
	wg.Add(4)
	go worker(fizz, func(int) { sb.WriteString("fizz ") })
	go worker(buzz, func(int) { sb.WriteString("buzz ") })
	go worker(fb, func(int) { sb.WriteString("fizzbuzz ") })
	go worker(num, func(i int) { fmt.Fprintf(&sb, "%d ", i) })
	for i := 1; i <= n; i++ {
		switch {
		case i%15 == 0:
			fb <- i
		case i%3 == 0:
			fizz <- i
		case i%5 == 0:
			buzz <- i
		default:
			num <- i
		}
		<-ack
	}
	close(fizz)
	close(buzz)
	close(fb)
	close(num)
	wg.Wait()
	return strings.TrimSpace(sb.String())
}

func main() { fmt.Println(fizzBuzz(15)) }
