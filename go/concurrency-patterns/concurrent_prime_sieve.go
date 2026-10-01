//go:build ignore

// Concurrent Prime Sieve (Hoare / Go tour classic)
// Statement: Generate the first n primes using a daisy chain of filter goroutines,
// one per discovered prime.
// Approach: a generator emits 2,3,4,...; each time a prime p reaches the end of the
// chain, append a filter stage that drops multiples of p. Done channel stops all.
// Time: roughly O(n^2 / log n) messages. Space: O(n) goroutines.
package main

import "fmt"

func generate(done <-chan struct{}) <-chan int {
	ch := make(chan int)
	go func() {
		defer close(ch)
		for i := 2; ; i++ {
			select {
			case ch <- i:
			case <-done:
				return
			}
		}
	}()
	return ch
}

func filter(done <-chan struct{}, in <-chan int, p int) <-chan int {
	out := make(chan int)
	go func() {
		defer close(out)
		for v := range in {
			if v%p == 0 {
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

func primes(n int) []int {
	done := make(chan struct{})
	defer close(done)
	ch := generate(done)
	res := make([]int, 0, n)
	for len(res) < n {
		p := <-ch
		res = append(res, p)
		ch = filter(done, ch, p)
	}
	return res
}

func main() { fmt.Println(primes(20)) }
