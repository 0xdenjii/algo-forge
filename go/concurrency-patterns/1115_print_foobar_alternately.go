//go:build ignore

// LeetCode 1115 - Print FooBar Alternately
// Statement: One goroutine prints "foo" n times, another prints "bar" n times.
// Make the combined output "foobar" repeated n times.
// Approach: token passing with two buffered channels; foo takes its token, prints,
// then hands a token to bar, and vice versa.
// Time: O(n). Space: O(1).
package main

import (
	"fmt"
	"strings"
	"sync"
)

type FooBar struct {
	n          int
	fooT, barT chan struct{}
}

func NewFooBar(n int) *FooBar {
	fb := &FooBar{n, make(chan struct{}, 1), make(chan struct{}, 1)}
	fb.fooT <- struct{}{}
	return fb
}

func (fb *FooBar) Foo(p func()) {
	for i := 0; i < fb.n; i++ {
		<-fb.fooT
		p()
		fb.barT <- struct{}{}
	}
}

func (fb *FooBar) Bar(p func()) {
	for i := 0; i < fb.n; i++ {
		<-fb.barT
		p()
		fb.fooT <- struct{}{}
	}
}

func main() {
	fb := NewFooBar(4)
	var sb strings.Builder
	var wg sync.WaitGroup
	wg.Add(2)
	go func() { defer wg.Done(); fb.Bar(func() { sb.WriteString("bar") }) }()
	go func() { defer wg.Done(); fb.Foo(func() { sb.WriteString("foo") }) }()
	wg.Wait()
	fmt.Println(sb.String())
}
