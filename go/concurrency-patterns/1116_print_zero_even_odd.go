//go:build ignore

// LeetCode 1116 - Print Zero Even Odd
// Statement: Three goroutines print zeros, even numbers and odd numbers. For n the
// output must be "0102030...0n".
// Approach: the zero goroutine drives the sequence: it prints 0 then signals the odd
// or even channel; that worker prints and signals zero back.
// Time: O(n). Space: O(1).
package main

import (
	"fmt"
	"strings"
	"sync"
)

type ZeroEvenOdd struct {
	n               int
	zero, even, odd chan struct{}
}

func New(n int) *ZeroEvenOdd {
	z := &ZeroEvenOdd{n, make(chan struct{}, 1), make(chan struct{}), make(chan struct{})}
	z.zero <- struct{}{}
	return z
}

func (z *ZeroEvenOdd) Zero(p func(int)) {
	for i := 1; i <= z.n; i++ {
		<-z.zero
		p(0)
		if i%2 == 1 {
			z.odd <- struct{}{}
		} else {
			z.even <- struct{}{}
		}
	}
}

func (z *ZeroEvenOdd) Even(p func(int)) {
	for i := 2; i <= z.n; i += 2 {
		<-z.even
		p(i)
		z.zero <- struct{}{}
	}
}

func (z *ZeroEvenOdd) Odd(p func(int)) {
	for i := 1; i <= z.n; i += 2 {
		<-z.odd
		p(i)
		z.zero <- struct{}{}
	}
}

func main() {
	z := New(7)
	var sb strings.Builder
	p := func(x int) { fmt.Fprint(&sb, x) }
	var wg sync.WaitGroup
	wg.Add(3)
	go func() { defer wg.Done(); z.Even(p) }()
	go func() { defer wg.Done(); z.Odd(p) }()
	go func() { defer wg.Done(); z.Zero(p) }()
	wg.Wait()
	fmt.Println(sb.String()) // 01020304050607
}
